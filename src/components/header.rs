use maud::{html, Markup};

pub fn header(page_title: &str) -> Markup {
  // let css_path = format!("/static/{}.css", page_title);

  html! {
    head {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1.0";
      meta http-equiv="X-UA-Compatible" content="ie=edge";

      // the style is hardcoded because it allows me to ship a single binary
      // without any other files around it.
      style type="text/css" {
        (master_css_content())
      }
  
      title { (page_title) }
    }
  }
}

fn master_css_content() -> String {
  "
  html, body {
    font-family: Bahnschrift;
    min-height: 100vh;
  
    padding: 0;
    margin: 0;
  }

  
  ".to_owned()
}