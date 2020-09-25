pub mod timer;

#[macro_export]
macro_rules! impl_responder {
    ($target:ident) => {
        impl actix_web::Responder for $target {
            type Error = actix_web::Error;
            type Future = futures::future::Ready<
                std::result::Result<actix_web::HttpResponse, actix_web::Error>,
            >;

            fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
                let body = serde_json::to_string(&self).unwrap();
                futures::future::ready(Ok(actix_web::HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)))
            }
        }
    };
}
