use r2d2_redis::RedisConnectionManager;

pub struct RedisStore {
    pub pool: r2d2_redis::r2d2::Pool<RedisConnectionManager>,
}

impl RedisStore {
    pub fn from_env() -> Self {
        let redis_url = match std::env::var("REDIS_URL") {
            Ok(url) => url,
            Err(e) => {
                tracing::error!("REDIS_URL must be set in .env: {:?}", e);
                std::process::exit(1);
            }
        };

        let manager = match RedisConnectionManager::new(redis_url) {
            Ok(manager) => manager,
            Err(e) => {
                tracing::error!("Failed to create RedisConnectionManager: {:?}", e);
                std::process::exit(1);
            }
        };
        let pool = match r2d2_redis::r2d2::Pool::new(manager) {
            Ok(pool) => pool,
            Err(e) => {
                tracing::error!("Failed to create Redis pool: {:?}", e);
                std::process::exit(1);
            }
        };

        Self { pool }
    }

    pub fn get_connection(&self) -> anyhow::Result<r2d2::PooledConnection<RedisConnectionManager>> {
        self.pool.get().map_err(|e| anyhow::anyhow!(e))
    }
}
