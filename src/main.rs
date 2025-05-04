use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("http://127.0.0.1:8000")?;
    run(listener)?.await
}
