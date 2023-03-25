pub struct Server;

impl Server {
    pub async fn initialize() -> std::io::Result<()> {
        let bind_ip = "0.0.0.0";
        let bind_port = 8080;

        println!("Starting server at {}:{}", bind_ip, bind_port);

        actix_web::HttpServer::new(super::configure::create_app)
            .client_request_timeout(std::time::Duration::from_millis(2500)) 
            .workers(8)
            .bind((bind_ip, bind_port)).unwrap()
            .run()
            .await
    }
}
