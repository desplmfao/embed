pub fn create_app() -> actix_web::App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Config = (),
        InitError = (),
        Error = actix_web::Error,
    >,
> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./static/templates")
        .map_err(|e| {
            println!("Failed to properly register handlebars templates: {}", e);
            e
        })
        .unwrap();

    let handlebars_ref = actix_web::web::Data::new(handlebars);

    let max_payload_size: i128 = 1024 * 1024 * 1024 * 1024;

    let client = awc::ClientBuilder::new()
        .max_http_version(awc::http::Version::HTTP_2)
        .disable_redirects()
        .no_default_headers()
        .max_redirects(0)
        .timeout(std::time::Duration::from_millis(2500))
        .finish();

    let client_data = actix_web::web::Data::new(client);

    actix_web::App::new()
        .configure(configure)
        .app_data(actix_web::web::PayloadConfig::new(max_payload_size.try_into().unwrap()))
        .app_data(handlebars_ref.clone())
        .app_data(client_data.clone())

        .service(
            actix_web::web::resource("/-")
                .to(super::routes::embed::embed)
        )

        .service(
            actix_web::web::resource("/-.{ext}")
                .to(super::routes::embed::embed)
        )
}

fn configure(
    _cfg: &mut actix_web::web::ServiceConfig
) {

}
