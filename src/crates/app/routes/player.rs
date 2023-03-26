pub async fn player(
    request: actix_web::HttpRequest,
    hb: actix_web::web::Data<handlebars::Handlebars<'_>>
) -> actix_web::HttpResponse {
    return actix_web::HttpResponse::NoContent().finish();
}