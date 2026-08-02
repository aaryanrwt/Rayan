use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use rayan_core::asg::Asg;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::net::SocketAddr;
use tracing::info;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Initialize SQLite database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS public_graph (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            asg_json TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    let state = AppState { pool };

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "Rayan Global Commons Engine OK" }),
        )
        .route("/publish", post(publish_field))
        .route("/search", get(search_fields))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting Rayan Cloud API on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn search_fields(State(state): State<AppState>) -> Json<Vec<String>> {
    use sqlx::Row;
    let mut results = Vec::new();
    if let Ok(records) = sqlx::query("SELECT asg_json FROM public_graph")
        .fetch_all(&state.pool)
        .await
    {
        for record in records {
            if let Ok(json) = record.try_get::<String, _>("asg_json") {
                results.push(json);
            }
        }
    }
    Json(results)
}

async fn publish_field(State(state): State<AppState>, Json(payload): Json<Asg>) -> String {
    info!(
        "Received publish request for ASG with {} nodes",
        payload.nodes.len()
    );
    let json_str = serde_json::to_string(&payload).unwrap();

    match sqlx::query("INSERT INTO public_graph (asg_json) VALUES (?)")
        .bind(json_str)
        .execute(&state.pool)
        .await
    {
        Ok(_) => "Successfully stored in Global Commons".to_string(),
        Err(e) => {
            tracing::error!("DB Insert Error: {}", e);
            "Internal Server Error".to_string()
        }
    }
}
