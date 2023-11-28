use actix_web::{get, App, web::Data, HttpServer, HttpRequest, Responder};
use actix_redis::{Command, Error, RedisActor, RespValue, resp_array};
use actix::prelude::Addr;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/ip")]
async fn ip(req: HttpRequest, redis_actor: Data<Addr<RedisActor>>) -> impl Responder {
    let set = redis_actor.send(Command(resp_array!["SET", "key", "value"])).await;

    let get = redis_actor.send(Command(resp_array!["GET", "key"])).await;
    match get {
        Ok(Ok(respValue)) => {
            println!("{:?}", respValue);
        },
        _ => panic!("Should not happen {:?}", get),
    }

    if let Some(val) = req.peer_addr() {
        format!("Address {:?}", val.ip())
    } else {
        "No IP Address found".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    println!("Actix Web Server is starting.");

    let redis_actor = RedisActor::start("redis:6379");

    HttpServer::new(
        move || App::new()
            .service(index)
            .service(ip)
            .app_data(Data::new(redis_actor.clone()))
        )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}