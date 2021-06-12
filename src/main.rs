use neo4rs::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use uuid::Uuid;

#[tokio::main]
async fn main() {
   let uri = "localhost:7687";
   let user = "test";
   let pass = "test";
   let id = Uuid::new_v4().to_string();

   let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
   let mut _result = graph.run(
     query("CREATE (p:Person {id: $id})").param("id", id.clone())
   ).await.unwrap();

   let mut handles = Vec::new();
   let count = Arc::new(AtomicU32::new(0));
   for _ in 1..=42 {
       let graph = graph.clone();
       let id = id.clone();
       let count = count.clone();
       let handle = tokio::spawn(async move {
           let mut result = graph.execute(
             query("MATCH (p:Person {id: $id}) RETURN p").param("id", id)
           ).await.unwrap();
           while let Ok(Some(_row)) = result.next().await {
               count.fetch_add(1, Ordering::Relaxed);
           }
       });
       handles.push(handle);
   }

   futures::future::join_all(handles).await;
   println!["{}", count.load(Ordering::Relaxed)];
}