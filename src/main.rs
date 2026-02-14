use api_productos::config::server::ServerConfig;
use api_productos::connexion::{pool::connection_pool, state::AppState};
use api_productos::routes::routes::routes;
use tokio::net::TcpListener as TCP;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Configuracion del servido<D-s><D-s>
    let config = ServerConfig::from_env()?;

    // 2. Conexión a base de datos
    let db_pool = match connection_pool(None).await {
        Ok(pool) => {
            info!("Database connection successfully...");
            pool
        }
        Err(e) => {
            error!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    };

    // configuracion del consumer
    let app_state = AppState { db: db_pool };

    // configuracion de las rutas
    let app = routes(app_state);

    // 5. Iniciar servidor
    let listener = TCP::bind(config.socket_addr()?).await?;
    info!("Serveres connection successfully {}", config.url());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
