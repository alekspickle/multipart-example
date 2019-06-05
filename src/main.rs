#![allow(unused)]

mod server;

use server::Server;
use std::env;

fn main() {
    //set RUST_LOG enviroment variable to enable logs from actix_web
    env::set_var("RUST_LOG", "actix_web=info");
    
    //  init logger
    //  env_logger::init();
    let port = env::var("PORT").unwrap_or_else(|_e| "3000".into());

    //all logic inside Server struct
    let mut server_1 = Server {
        name: "server_1".into(),
        address: "0.0.0.0".into(),
        port: port,
    };

    server_1.start();
}

