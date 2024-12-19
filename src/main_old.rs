use std::{collections::HashMap, fmt::Display};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum AppResponseStatus {
    Success,
    Fail,
}

#[derive(Deserialize, Serialize)]
struct AppResponse<T> {
    data: T,
    status: AppResponseStatus,
}

impl<T> AppResponse<T> {
    pub fn new(data: T, status: AppResponseStatus) -> Self {
        Self { data, status }
    }
}

#[derive(Deserialize, Serialize)]
struct IndexHandlerReturn {
    welcome: String,
}

#[derive(Deserialize, Serialize)]
struct EnterNickHandlerReturn {
    nickname: String,
}

#[derive(Deserialize, Serialize)]
struct GetGameBoardsHandlerReturn {
    boards: Vec<GameBoard>,
}

#[derive(Deserialize, Serialize)]
struct GameBoard {
    peer_count: usize,
    peers: HashMap<usize, String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/enter_nick", get(enter_nick_handler))
        .route("/get_game_boards", get(get_game_boards_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

type HandlerResult<T> = Result<T, axum::Error>;

async fn index_handler() -> String {
    let data = IndexHandlerReturn {
        welcome: "Welcome to our game.".to_string(),
    };

    serde_json::to_string(&data).unwrap()
}

#[derive(Clone, Deserialize)]
struct EnterNickQueryParams {
    username: String,
}

#[derive(Clone, Deserialize, Debug)]
struct PaginationQueryParams {
    page: usize,
    per_page: usize,
}

async fn enter_nick_handler(params: Query<EnterNickQueryParams>) -> String {
    let query_params = params.0.clone();

    let data = EnterNickHandlerReturn {
        nickname: query_params.username.clone(),
    };

    serde_json::to_string(&data).unwrap()
}

async fn get_game_boards_handler(params: Query<PaginationQueryParams>) -> String {
    let pagination_params = params.0.clone();
    println!("pagination_params: {:?}", pagination_params);

    let mut boards: Vec<GameBoard> = Vec::new();

    let mut peers: HashMap<usize, String> = HashMap::new();
    peers.insert(1, "Ahmet".to_string());
    peers.insert(0, "Emir".to_string());
    peers.insert(2, "Ataberk".to_string());
    boards.push(GameBoard {
        peers,
        peer_count: 4,
    });

    let mut peers: HashMap<usize, String> = HashMap::new();
    peers.insert(3, "Hakkı".to_string());

    boards.push(GameBoard {
        peers,
        peer_count: 4,
    });

    let data = GetGameBoardsHandlerReturn { boards };

    serde_json::to_string(&data).unwrap()
}
