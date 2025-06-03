# Errors and their resolve

## the trait `App<impl ServiceFactory<{type error}>>:actix_service::IntoServericeFactoru is not satisfied`

`actix_service = '0.4'`
use actix_service::IntoServiceFactory;

## the method bind exeists for struct HttpServer but its trait bounds wre not satisfied

HttpServer::new(move || {
create_http_server(mongo_client.clone(), pg_pool.clone()) 
})
.bind("127.0.0.1:8080")?  


