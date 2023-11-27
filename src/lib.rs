mod routes;

use routes::create_routes;
use tokio::net::TcpListener;

pub async fn run() {
    let app = create_routes();
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
