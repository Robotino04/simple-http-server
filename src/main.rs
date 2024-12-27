use warp::Filter;
use std::{env, path::PathBuf};
use dotenv;
use env_logger;

async fn get_env_variable(env_variable: String) -> Result<Box<dyn warp::Reply>, warp::Rejection>  {
    match env::var(env_variable) {
        Ok(env) => Ok(Box::new(env)),
        Err(_) => Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    }
}

#[tokio::main]
async fn main() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let mut exe_path = env::current_exe().expect("Failed to get binary location");
    exe_path.pop();

    dotenv::dotenv().expect("Failed to load .env file.");

    let port: u16 = dotenv::var("PATH_OVERRIDE")
        .map(|s| s.parse::<u16>().expect("PATH_OVERRIDE in .env file is not a number"))
        .unwrap_or(7272);
    let initial_path: String = dotenv::var("INITIAL_PATH")
        .unwrap_or("index.html".to_string());
    let static_file_location: PathBuf = dotenv::var("STATIC_FILE_PATH")
        .map(|s| s.into())
        .unwrap_or(exe_path.to_owned());


    let files = warp::get()
        .and(warp::fs::dir(static_file_location.clone()))
        .with(warp::log("files"));

    let env = warp::path!("$env" / String)
        .and_then(get_env_variable)
            .with(warp::log("env"));

    println!("Serving {static_file_location:#?} on 'http://0.0.0.0:{port}/'.");
    let server = warp::serve(files.or(env))
        .run(([0, 0, 0, 0], port));

    let url_to_open = std::format!("http:/localhost:{port}/{initial_path}");
    println!("Opening '{url_to_open}' in browser.");
    open::that(url_to_open).expect("Failed to open browser");

    server.await;
}
