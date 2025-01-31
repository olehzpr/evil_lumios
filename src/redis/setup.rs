pub fn establish_connection() -> redis::Client {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("REDIS_URL must be set in .env: {:?}", e);
            std::process::exit(1);
        }
    };
    let client = match redis::Client::open(redis_url) {
        Ok(client) => client,
        Err(e) => {
            tracing::error!("Failed to create redis client: {:?}", e);
            std::process::exit(1);
        }
    };

    tracing::info!("Successfully connected to redis");

    client
}
