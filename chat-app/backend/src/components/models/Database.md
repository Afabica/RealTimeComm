# Database 

## format! - String Formatting Macro.
- Purpose: used to dynamically format strings at runtime.
- Use Case in SQL Quueries: Helps construct dynamic queries safely. 

## bind() - SQL query parameter binding.
- Purpose: Securely binds values to SQL queries, preventing SQL injection.
- Use Case: When querying with dynamic values.

## fetch_one() - Fetch a single record.
- Purpose: retrieves a sinagle row from the database.
- Use casse: When expecting exactly one result(fetching a user by ID)

## execute() - run quaries that don't return rows.
- Purpose: used for quaries like INSERT, UPDATE, or DELETE that don't return rows.
- Use case: When updating a user's email.
Example:
<mark>
async fn update_user_email(pool: &sqlx::PgPool, user_id: i32, new_email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE login_users SET email = $1 WHERE id = $2")
        .bind(new_email)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}


</mark>

## fetch_optional() - fetch one or none
- Purpose: Returns either one row or none.
- Use Case: When a user may or may not exist.
Example:
<mark>

async fn find_user_by_email(pool: &sqlx::PgPool, email: &str) -> Result<Option<LoginUser>, sqlx::Error> {
    let user = sqlx::query_as::<_, LoginUser>("SELECT * FROM login_users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}


</mark>

## try_into() - Convert Query Results to Structs 
- Purpose: Converts database rows into Rust sttructs
- Use Case: When fetching raw rows and converting manually

Example: 
<mark>
use sqlx::Row;

async fn get_username(pool: &sqlx::PgPool, user_id: i32) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT username FROM login_users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    
    let username: String = row.try_get("username")?;
    Ok(username)
}


</mark>

## begin() - Transaction Support
- Purpose: Allows executing multiple queries within a single transaction.
- Use Case: Ensures atomic operations.
Example: 
<mark>

async fn transfer_credits(pool: &sqlx::PgPool, from_user: i32, to_user: i32, amount: i32) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    
    sqlx::query("UPDATE users SET credits = credits - $1 WHERE id = $2")
        .bind(amount)
        .bind(from_user)
        .execute(&mut transaction)
        .await?;

    sqlx::query("UPDATE users SET credits = credits + $1 WHERE id = $2")
        .bind(amount)
        .bind(to_user)
        .execute(&mut transaction)
        .await?;

    transaction.commit().await?;
    Ok(())
}


</mark>

## Example of combincation of PostgreSQL & MongoDB in Rust

You can use both databases in the same app:
- PostgreSQL -> Structured data (userr, transaction)
- MongoDB -> Flexible data (chat messages, logs)

Example: 

<mark>
async fn store_chat_message(
    postgres_pool: &PgPool,
    mongo_collection: &Collection<User>,
    username: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Save in PostgreSQL (User exists check)
    let user = get_user_by_username(mongo_collection, username).await?;
    if user.is_none() {
        return Err("User not found".into());
    }

    // Save chat message in MongoDB
    let chat_doc = doc! {
        "username": username,
        "message": message,
        "timestamp": chrono::Utc::now().to_string()
    };
    mongo_collection.insert_one(chat_doc, None).await?;

    Ok(())
}
- Checks if the user exists in PostgreSQL.
- Savs the chat message in MongoDB.

To given function is possible to implement next steps:
- add transactions in PostgreSQL (.begin_transaction())
- implement full-text search in MongoDB ($text query)
- Optimize querires for performance.

</mark>
