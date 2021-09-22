use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::process::Command;

fn extract_ip_addr (txt: &str) -> Option<String> {
    lazy_static! {
        static ref IP_RE: Regex = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    }
    
    match IP_RE.find(txt) {
        None => None,
        Some(m) => Some(m.as_str().to_owned())
    }
}

fn get_ip_address () -> Option<String> {
    let output = Command::new("surfshark-vpn")
        .arg("status")
        .output()
        .expect("failed to query surfshark");
    let stdout: Vec<u8> = output.stdout;
    let txt: String = String::from_utf8(stdout).unwrap();
    
    return extract_ip_addr(txt.as_str())
}

#[get("/")]
async fn ss_status() -> impl Responder {
    match get_ip_address() {
        None => HttpResponse::Ok().body( "OFF []" ),
        Some(ip) => HttpResponse::Ok().body( format!("ON [{}]", ip) ),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    let port: u16;

    if args_len == 2 {
        port = args[1].parse().unwrap();

        if port < 12000 {
            println!("port number shoud be 12000 or above");
            std::process::exit(1);
        }
    } else {
        println!("please, inform port to listen");
        std::process::exit(1);
    }
    

    HttpServer::new(|| {
        App::new()
            .service(ss_status)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}