// ########### original idea from shirtjs ###########

pub async fn embed(
    request: actix_web::HttpRequest,
    hb: actix_web::web::Data<handlebars::Handlebars<'_>>
) -> actix_web::HttpResponse {
    let q_array = request.query_string().split("url=");
    let q_urlr = q_array.collect::<Vec<_>>()[1];

    if q_urlr.len() <= 11 {
        return actix_web::HttpResponse::NoContent().finish();
    }

    let q_url = urlencoding::decode(q_urlr).unwrap();

    if request.uri().path_and_query().unwrap().to_string().contains(&"/-?url=") {
        let data = serde_json::json!({
            "url": ("/-.mp4?url=".to_owned() + &q_url.to_string()),
            "media_url": urlencoding::decode(&q_url.to_string()).unwrap()
        });

        let body = hb.render("embed", &data).unwrap();
    
        actix_web::HttpResponse::Ok().body(body)
    } else {
        return actix_web::HttpResponse::PermanentRedirect()
            .append_header(("location", urlencoding::decode(&q_url.to_string()).unwrap().to_string()))
            .finish()
    }
}