// ########### original idea from shirtjs ###########

pub async fn embed(
    client: actix_web::web::Data<awc::Client>,
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
            "url": ("/-.mp4?url=".to_owned() + &q_url.to_string())
        });

        let body = hb.render("embed", &data).unwrap();
    
        actix_web::HttpResponse::Ok().body(body)
    } else {
        let mut response = match client
            .request(
                actix_web::HttpRequest::method(&request).clone(),
                q_urlr.to_string().to_owned(),
            )
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                println!("{:?}", e);
                return actix_web::HttpResponse::InternalServerError().finish();
            }
        };

        return actix_web::HttpResponse::Ok()
            .body(
                match response
                    .body()
                    .limit(1024 * 1024 * 512)
                    .await
                {
                    Ok(resp) => resp.to_vec(),
                    Err(e) => {
                        println!("{:?}", e);
                        return actix_web::HttpResponse::InternalServerError().finish();
                    }
                },
            );
    }
}
