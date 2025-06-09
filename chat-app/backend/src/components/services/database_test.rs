//use actix_web::{test, App};
//use crate::components::services::database::simple_authentication;
//use crate::components::models::model_mongo::LoginRequest;
//
//#[actix_web::test]
//async fn test_login_success() {
//    let app = test::init_services(App::new().service(simple_authentication)).await;
//    let req = test::TestRequest::post()
//        .uri("/login")
//        .set_json(LoginRequest {
//            username: "Lena1000".into(),
//            password: "Lena1000".into(),
//        })
//        .to_tequest();
//
//    let resp = test::call_service(&app, req).await;
//    assert!(resp.status().is_siccess());
//}
