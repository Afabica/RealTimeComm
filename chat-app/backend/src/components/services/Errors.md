# Errors in code

## impl Responder: the trait `actix_web::Responder` is not implemented for '()'.

Error occuring from problem like function that printing data never return anyhtin that implements Responder. So the compiler complains, because it's implicitly returning ().

## thead 'main' panicked at mani.rs called `Result::unwrap()` on an Err value: Io (Custom {kind: uncategorized, error failed to lookup address information: name or service not known})

App crashed in the main thread. Called `unwrap()` on a function that returned a Result::Err(...) .unwrap() panics if the result isn't Ok(...).

- Io error: failed to lookup address information: name or service not known.
  This is usually caused by:
- Trying to bind your Actix server to a bad address, like "localhost:8080" when "localhost" isn't resolving.
- Or trying to connect to a database using a URL with a hostname like "db" or "postgres" that your system can't resolve.

## Authentication Function

<mark> 
pub async fn simple_authentication(pg_pool: web::Data<PgPool>, credentiales: web::Json<LoginRequest>) -> Result<HttpResponse, sqlx::Error>  {
    let login = credentiales.into_inner();
    let query = format!("SELECT password FROM users WHERE username = {}", &login.username);
    let mut rows = sqlx::query(&query).fetch(pg_pool);

    match rows {
        Ok(Some(records)) => {
            if verify(&login.password , &records.password).unwrap_or(false) {
                Ok(HttpResponse::Ok().body("Authenticaiton successful"))
            } else {
                Ok(HttpResponse::Unauthorized().body("Invalid password"))
            }
        },
        Ok(None) => HttpResponse::Unauthorized().body("User not found"),
        Err(e) => {
            eprintln!("Databse error: {:?}", e);
            Err(HttpResponse::InternalServerError().body("Internal server error"))
        }
    }

}

### Codes error

- SQL Injection risk: Use of `format! to inject user input into the query string directly - this is a security vulnerability. Use sqlx::query! or parameterized queries to prevent this`
- Incorrect fetch() usage: You used .fetch() which returns a stream, not a result. You want to use .fetch_optional().await
- Wring match target: rows is a Stream, but tried to match it as if it's already a result.
- Wrong return types in match arms: You're returning HttpResponse in some arms, but not wrapping them in Ok().
- Cna't return HttpResponse::Unauthorized directly in Result<HttpResponse, sqlx::Erro> without Ok(...)

</mark>

## he trait bound `&Pool<Postgres>: actix_web::FromRequest` is not satisfied 
means that handler function is incorrectly tryung to extract a reference to the Postgres pool like &PgPool directly, but Actix supports only extracting web::Data<PgPool>
