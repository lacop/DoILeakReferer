use askama::Template;
use warp::Filter;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "test.html")]
struct TestTemplate;

enum Referrer {
    NotPresent,
    NotValidUTF8,
    Present(String),
}

#[derive(Template)]
#[template(path = "image.svg", escape = "xml")]
struct ImageTemplate {
    referrer: Referrer,
}

fn image(headers: warp::http::HeaderMap) -> ImageTemplate {
    let referrer = match headers.get(warp::http::header::REFERER) {
        Some(r) => match std::str::from_utf8(r.as_bytes()) {
            Ok(s) => Referrer::Present(s.to_owned()),
            Err(_) => Referrer::NotValidUTF8,
        },
        None => Referrer::NotPresent,
    };

    ImageTemplate { referrer }
}

#[tokio::main]
async fn main() {
    // Matches /image or /image/whatever.
    let image = warp::path("image")
        .and(warp::filters::header::headers_cloned())
        .map(image);
    let test = warp::path("test").map(|| TestTemplate {});
    let index = warp::any().map(|| IndexTemplate {});

    // TODO: Switch between image or index for root based on accept header?
    let routes = image.or(test).or(index);

    warp::serve(routes)
        .run((std::net::Ipv6Addr::UNSPECIFIED, 8080))
        .await;
}
