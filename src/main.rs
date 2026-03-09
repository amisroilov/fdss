use hyper::{Request, Response, Body, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::fs;
use std::path::Path;
use mime_guess::from_path;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("Method not allowed"))
            .unwrap());
    }

    let mut path = format!(".{}", req.uri().path());

    if path == "./" {
        path = "./index.html".to_string();
    }

    if Path::new(&path).is_dir() {
        let index_html = format!("{}/index.html", path);
        let index_htm = format!("{}/index.htm", path);

        if Path::new(&index_html).exists() {
            path = index_html;
        } else if Path::new(&index_htm).exists() {
            path = index_htm;
        }
    }

    match fs::read(&path) {
        Ok(contents) => {
            let mime = from_path(&path).first_or_octet_stream();

            Ok(Response::builder()
                .header("Content-Type", mime.as_ref())
                .body(Body::from(contents))
                .unwrap())
        }
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 - File Not Found"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let addr = ([127,0,0,1], 8080).into();

    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    println!("FDSH running at http://localhost:8080");

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}