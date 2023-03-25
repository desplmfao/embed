pub mod crates;

#[actix_web::main]
async fn main() {
    crate::crates::app::server::Server::initialize()
        .await
        .expect("ERR(server init)");
}