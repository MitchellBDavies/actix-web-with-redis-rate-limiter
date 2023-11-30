use actix_web::{get, App, web::Data, HttpServer, HttpRequest, HttpResponse, Responder, http::StatusCode};
use actix_redis::{Command, RedisActor, resp_array};
use actix::prelude::Addr;
use std::net::IpAddr;

use std::time::SystemTime;

enum RateLimitResult {
    RateLimit,
    InternalError,
    NotLimited
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

fn redis_schema_ip_sount_set(ip_address: IpAddr) -> String {
    "ip_count_set:".to_owned() + &ip_address.to_string()
}

fn redis_schema_max_requests() -> i64 {
    10
}

fn redis_schema_rate_limit_window() -> i64 {
    30000
}

async fn check_ip_rate_limit(ip_address: IpAddr, redis_actor: Data<Addr<RedisActor>>) -> RateLimitResult {
    //let set = redis_actor.send(Command(resp_array!["SET", "key", "value"])).await;

    let ip_count_key = redis_schema_ip_sount_set(ip_address);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64;
    let window_start = current_time - redis_schema_rate_limit_window();
    let ip_count_in_window = redis_actor.send(Command(resp_array!["ZCOUNT", &ip_count_key, window_start.to_string(), current_time.to_string()])).await;

    println!("{:?}", ip_count_in_window);

    match ip_count_in_window {
        Ok(Ok(redis_async::resp::RespValue::Integer(requests_in_window))) => {
            if requests_in_window < redis_schema_max_requests() {
                let _ = redis_actor.send(Command(resp_array!["ZADD", &ip_count_key, current_time.to_string(), current_time.to_string()])).await;
                let _ = redis_actor.send(Command(resp_array!["ZREMRANGEBYSCORE", &ip_count_key, "-inf", window_start.to_string()])).await;
                RateLimitResult::NotLimited
            } else {
                RateLimitResult::RateLimit
            }            
        },
        _ => RateLimitResult::InternalError
    }
}

#[get("/ratelimit")]
async fn ip(req: HttpRequest, redis_actor: Data<Addr<RedisActor>>) -> impl Responder {

    if let Some(val) = req.peer_addr() {
        let ip_address = val.ip();
        match check_ip_rate_limit(ip_address, redis_actor).await {
            RateLimitResult::NotLimited  => {
                HttpResponse::Ok()
                    .body(format!("Address {:?}", ip_address))
            },
            RateLimitResult::RateLimit => {
                HttpResponse::new(StatusCode::TOO_MANY_REQUESTS)
            },
            RateLimitResult::InternalError => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
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