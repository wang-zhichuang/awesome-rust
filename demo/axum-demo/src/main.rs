use axum::{
  extract::{ContentLengthLimit, Multipart},
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();
  // build our application with a route
  let app = Router::new()
      // `GET /` goes to `root`
      .route("/", get(root))
      // `POST /users` goes to `create_user`
      .route("/users", post(create_user))
      .route("/call", post(call))
      .route("/download", post(create_user))
      .route("/upload", post(create_user));

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
  "Hello, World!"
}

async fn create_user(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
  // insert your application logic here
  let user = User {
      id: 1337,
      username: payload.username,
  };

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
struct CreateUser {
  username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
  id: u64,
  username: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CallRequest {
  body: String,
  accessToken: String
}
// call 
async fn call(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  Json(payload): Json<CallRequest>,
) -> impl IntoResponse {
  // insert your application logic here
  let callRequest = CallRequest {
      body: payload.body,
      accessToken: payload.accessToken,
  };
  tracing::debug!("callRequest  {:?}", callRequest);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(callRequest))
}



/// 允许上传的大小
const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10; // 10MB
async fn upload_file_action(
  ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
) {
  if let Some(file) = multipart.next_field().await.unwrap() {
      let filename = file.file_name().unwrap().to_string(); // 上传的文件名
      let data = file.bytes().await.unwrap(); // 上传的文件的内容

      // 保存上传的文件
      //std::fs::write(&filename, &data).map_err(|err| err.to_string())?;
      tokio::fs::write(&filename, &data)
          .await
          .map_err(|err| err.to_string());

  }
}