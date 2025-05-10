use std::net::TcpListener;

use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use email_newsletter::{configuration::get_configuration, email_client::EmailClient};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    run(listener, connection_pool, email_client)?.await
}
