use hyper::{Request, Response, Body, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::fs;
use std::path::Path;
use mime_guess::from_path;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Folder to serve
    #[arg(default_value = ".")]
    dir: String,

    /// Port number
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

async fn handle(
    req: hyper::Request<hyper::Body>,
    dir: String,
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    use std::path::Path;
    use std::fs;
    use mime_guess::from_path;
    use hyper::{Body, Response, Method, StatusCode};

    println!("[{}] {}", req.method(), req.uri().path());

    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("Method not allowed"))
            .unwrap());
    }

    let mut path = format!("{}{}", dir, req.uri().path()).replace("..", "");

    if Path::new(&path).is_dir() {
        if Path::new(&format!("{}/index.html", path)).exists() {
            path = format!("{}/index.html", path);
        } else if Path::new(&format!("{}/index.htm", path)).exists() {
            path = format!("{}/index.htm", path);
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
            .body(Body::from("<h1>404 - File not found</h1>"))
            .unwrap()),
    }
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("FDSS running at http://localhost:{}", args.port);
    println!("Serving directory: {}", args.dir);

    let addr = ([127, 0, 0, 1], args.port).into();

    let make_service = hyper::service::make_service_fn(|_| async {
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(|req| {
            handle(req, args.dir.clone())
        }))
    });

    let server = hyper::Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}