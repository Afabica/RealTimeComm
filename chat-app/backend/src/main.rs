use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
mod components;
use components::services::auth_reg::{iterate_mongodb_collection, iterate_postgres_collection, connect_to_mongodb, connect_to_postgres, check_mongo_connection, ping};
use components::services::auth_reg::{simple_authentication, simple_registration, get_specific_user_information}; 
use components::services::clserver::ws_index;
use components::models::model_mongo::{AppState, AppSettings};
use components::services::clserver_entities::ChatServer;
use components::servers::signaling_serv::ws_handler;
use rocket::yansi::Paint;
use actix::Actor;
use dotenvy::dotenv;
use std::env;
use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let local = LocalSet::new();
        
    local.run_until(async { 

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URL must be set.");
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "DATABASE_URL must be set"));
        }
    };

    let pg_pool = match connect_to_postgres().await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Error connecting to PostgreSQL: {:?}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "PostgreSQL connection error"));
        }
    };
    println!("Connected to PostgreSQL database");

    let pool_data = web::Data::new(pg_pool.clone());

    let mongo_client = match connect_to_mongodb().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error connecting to MongoDB: {:?}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "MongoDB connection error"));
        }
    };
    println!("Connected to MongoDB database");

    let chat_server = ChatServer::new().start();


    let mongo_clone = mongo_client.clone();
    let collection_name = "users".to_string();
    let table_name = "users".to_string();

    if let Err(err) = iterate_postgres_collection(&pg_pool, &collection_name).await {
        eprintln!("PostgreSQL Iteration Error: {:?}", err);
    } else {
        println!("PostgreSQL Iteration Completed.");
    }

    if let Err(err) = check_mongo_connection(mongo_clone.clone()).await {
        eprintln!("MongoDB Connection Error: {:?}", err);
    } else {
        println!("MongoDB still connected.");
    }

    if let Err(err) = iterate_mongodb_collection(mongo_clone.clone(), &table_name).await {
        eprintln!("MongoDB Iteration Error: {:?}", err);
    } else {
        println!("MongoDB Iteration Completed");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                mongo_client: mongo_client.clone(), 
                pg_pool: pool_data.clone(),
                chat_server: chat_server.clone(), 
            })) 
            .wrap(Logger::default())
            .route("/ws/", web::get().to(ws_handler))
            .route("/login", web::post().to(simple_authentication))

            .route("/registration", web::post().to(simple_registration))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    }).await
//    let _ =  create_http_server();
//
//    Ok(())
}

