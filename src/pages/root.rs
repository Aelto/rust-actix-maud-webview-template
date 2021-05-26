use crate::components;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {
    
    section {
      "hello world!"
    }

    style type="text/css" { (get_stylesheet()) }
  };

  let view = components::page("templates", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}

fn get_stylesheet() -> String {
  "
    
  ".to_owned()
}