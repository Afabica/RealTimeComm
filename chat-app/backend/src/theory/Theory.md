# Theory for Rust 

## Loading Environment Variables. 
use dotenvy::dotenv;
dotenv().ok()
- dotenv loads .env variables so credentials aren't haedcoded in the code
- The .env file should contain 

- PgPool::connect() creates a connection pool to PostgreSQL.
- Connection pooling helps reuse database connections instead of opening a new one each time.
- expect("msg") - ensures the app stops if the envirnoments variable is missing.

## Connecting to MongoDB 
- Arc<PgPool> -> PostgreSQL connection pool is shared across multiple threads.
- Arc<Mutex<Client>> -> MongoDB client is wrapped in a Mutex so mutliple threads can safely acess it.
- Arc (Atomic Reference Counting) allows multiple references it the same object without ownership conflicts.

## Example API Handler (Ping MongoDB)
Example:
<mark>
async fn ping_mongo(data: web::Data<AppState>) -> impl Responder {
    let client = data.mongo_client.lock().await;
    let db = client.database("test");

    match db.run_command(doc! {"ping": 1}, None).await {
        Ok(_) => HttpResponse::Ok().body("MongoDB is connected"),
        Err(err) => HttpResponse::InternalServerError().body(format!("MongoDB error: {}", err)),
    }
}
- Locks the MongoDB client (mongo_client.lock().await ensuring safe access.
- Sends a "ping" command to check if the database is alive.
- Returns an HTTP response:
1. "MongoDB is connected" if successsful
2. "MongoDB error: ..." if there;s an issue.
</mark>

## Use of Arc<PgPool> for PostgreSQL
- PgPool is threead-safe and can be shared withouht needing a Mutex.
- PostgreSQL connection pooling ensures:
- Reusing database connections.
- Avoiding overhead of opening new connections repeatedly.
- Arc helps share the pool efficiently across multiple threads.
