use axum::Router;
use decap_cms_oauth::AppState;
use decap_cms_oauth::router::oauth_router;
use std::env;
use std::process::exit;
use tokio::net::TcpListener;

fn check_var(var: &str) {
    if env::var(var).is_err() {
        eprintln!("error: undefined environment variable `{}`.", var);
        exit(1);
    }
}

#[tokio::main]
async fn main() {
    check_var("OAUTH_CLIENT_ID");
    check_var("OAUTH_SECRET");
    check_var("OAUTH_ORIGINS");

    let http_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build HTTP client");
    let state = AppState::new(http_client);
    let app = Router::new().merge(oauth_router(state));

    // Read port from environment variable `PORT`, default to 3005.
    let port: usize = match env::var("PORT") {
        Ok(s) => match s.parse::<usize>() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Provided PORT is not an integer");
                exit(1);
            }
        },
        Err(_) => 3005,
    };

    // Read bind address from `ADDRESS`. If ADDRESS contains a port (host:port), use it as-is.
    // Otherwise combine ADDRESS and PORT. Default ADDRESS is "127.0.0.1".
    let addr_env = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_address = if addr_env.contains(':') {
        addr_env
    } else {
        format!("{}:{}", addr_env, port)
    };

    let listener = TcpListener::bind(bind_address.clone()).await.unwrap();

    println!("Server listening on {}...", bind_address);

    axum::serve(listener, app).await.unwrap();
}
