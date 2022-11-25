use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use dy::{jwt_auth::server_run, log_init::log_writer_init};

#[tokio::main]
async fn main() {
    let file_name = log_writer_init().unwrap();
    //将filename转化为md作为web服务

    server_run().await;
}
