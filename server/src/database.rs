use mongodb::{Client, Database, options::Credential};

pub async fn connect() -> Option<Database> {
    let client_result = match (
        std::env::var("DATABASE_USERNAME"),
        std::env::var("DATABASE_PASSWORD"),
    ) {
        (Ok(username), Ok(password)) => {
            let credential = Credential::builder()
                .username(username)
                .password(password)
                .source("admin".to_string())
                .build();

            let options = mongodb::options::ClientOptions::builder()
                .credential(credential)
                .build();

            Client::with_options(options)
        }
        _ => Client::with_uri_str(std::env::var("DATABASE_URI").unwrap()).await,
    };

    match client_result {
        Ok(client) => Some(client.database("scm")),
        _ => None,
    }
}
