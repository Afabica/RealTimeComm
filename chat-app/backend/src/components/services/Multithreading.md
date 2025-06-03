# Multithreading

`std::thread` - standars library for use threads.
`std::time::Duration`

## Functions and examples

`std::thread::spawn` - lets you create new threads easily.
`std::thread::sleep` - simulates work, allowing threads to switch execution.
Example of code:
<mark>

use std::thread;

fn main() {
let handle = thread::spawn(|| {
for i in 1..5 {
println!("✅ Thread: {}", i);
}
});

    handle.join().unwrap(); // Waits for thread to finish before continuing
    println!("🎉 Main thread completed!");

}

</mark>
- handle.join().unwrap() blocks until the spawned thread finishes.
- Without `.join()`, the main thread could exit early.
`If spawned thread needs access to variables from the main thread, use move.`

Example:

<mark>

use std::thread;

fn main() {
let message = String::from("Hello from main!");

    let handle = thread::spawn(move || {
        println!("🧵 Thread says: {}", message);
    });

    handle.join().unwrap();

}

</mark>

- move transfers ownership of message into the thread.
- Without move, Rust won't allow message to be used inside the thread.

## Using `Arc<Mutex<T>> for shared data`

<mark>

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
let counter = Arc::new(Mutex::new(0)); // Shared counter with thread-safe access
let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock mutex
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Counter: {}", *counter.lock().unwrap());

}

</mark>

- Mutex<T> - provides safe access to data.
- Arc<T> - (Atomic Reference Counter) enables shared ownership across threads.
- .lock().unwrap() locks the mutex before modifying the data.
  `For easier parallel computing rayon`

Parallel Processing Example:

<mark>

use rayon::prelude::\*;

fn main() {
let numbers: Vec<i32> = (1..=10).collect();

    let squared_numbers: Vec<i32> = numbers.par_iter().map(|&x| x * x).collect();

    println!("Squared Numbers: {:?}", squared_numbers);

}

</mark>

- .par_iter() enables parallel iteration across multiple CPU cores.
- map(|x| x \* x) runs in parallel.
- .join ensure thread completes
- move keyword transfer ownership to thread
- thread-safe counter
- rayi=on vector parallelism

## Database

1. Different Data Workloads

- MongoDB: Handles unstructured or semi-structured data (e.g., chat messages, logs, JSON documents)

2. Parallel Processing & Performance

- Running MongoDB in one thread and PostgreSQL in another allows handling different database operations in parallel instead of blocking execution.
- Examlple: A chat app where MongoDB stores chat history while PostgreSQL handles user authentication.

### Better Approach

Instead of creating separate threads, a better approach is using Rust's aasync runtime (tokio or async-std):
Example:
<mark>

use mongodb::{Client as MongoClient, options::ClientOptions};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// ✅ Connect to PostgreSQL asynchronously
let pg_pool = PgPoolOptions::new()
.max_connections(5)
.connect("postgres://user:password@localhost/mydb")
.await?;

    // ✅ Connect to MongoDB asynchronously
    let mongo_client = MongoClient::with_options(ClientOptions::parse("mongodb://localhost:27017").await?)?;

    // ✅ Use both connections in async tasks
    tokio::join!(
        fetch_postgres_data(&pg_pool),
        fetch_mongo_data(&mongo_client)
    );

    Ok(())

}

async fn fetch_postgres_data(pool: &PgPool) {
// Example PostgreSQL query
let row: (i32,) = sqlx::query_as("SELECT COUNT(\*) FROM users")
.fetch_one(pool)
.await
.unwrap();
println!("👤 Total Users: {}", row.0);
}

async fn fetch_mongo_data(client: &MongoClient) {
let database = client.database("chatapp");
let collection = database.collection::<mongodb::bson::Document>("messages");

    let count = collection.count_documents(None, None).await.unwrap();
    println!("💬 Total Messages: {}", count);

}

</mark>

Reasons why better:

- No manual threads: uses tokio::join! to execute MongoDB & PostgreSQL queries concurrently.
- Efficient Resource Usage: Async tasks avoid unnecessary CPU overhead from thread switching.
- Scalable: You can handle thousands of connections efficiently.
