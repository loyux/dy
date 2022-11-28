//! Example JWT authorization/authentication.
//!
//! Run with
//!
//! ```not_rust
//! JWT_SECRET=secret cargo run -p example-jwt
//! ```

///jwt认证模式，首先获得token，然后通过token
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Form, Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;

use crate::req_api::{list_elems, Dylist, Dyurl, Elp};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt::Display, net::SocketAddr};

// Quick instructions
//
// - get an authorization token:
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -d '{"client_id":"foo","client_secret":"bar"}' \
//     http://localhost:3000/authorize
//
// - visit the protected area using the authorized token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
//
// - try to visit the protected area using an invalid token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer blahblahblah' \
//     http://localhost:3000/protected

// static KEYS: &str = "a";
pub async fn healthy_check() -> impl IntoResponse {
    let time_now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    let rp = format!("time: {}, Healthy", time_now);
    rp
}

pub async fn server_run_with_swagger() {
    let cc = ClientInfo {
        client_id: "foo".to_string(),
        client_secret: "bard".to_string(),
    };
    todo!();
}

pub async fn server_run() {
    let cc = ClientInfo {
        client_id: "foo".to_string(),
        client_secret: "bard".to_string(),
    };
    let app = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
        .route("/authform", post(authorize_form_data))
        .route("/healthy", get(healthy_check))
        //解决error[E0277]: the trait bound `fn(bool) -> impl Future {handler}: Handler<_, _>` is not satisfied
        //参考资料https://docs.rs/axum/latest/axum/handler/trait.Handler.html
        //引入特性 axum-macros
        //中间件结构体引入ClientInfo clone traits
        .layer(Extension(cc))
        .route("/dy", post(dy_info_list));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("Need JWT_SECRET env var");
    // let secret = "hello";
    Keys::new(secret.as_bytes())
});

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    dbg!(&claims);
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClientInfo {
    client_id: String,
    client_secret: String,
}
async fn authorize_form_data(
    Form(payload): Form<AuthPayload>,
    cc: Extension<ClientInfo>,
) -> Result<Json<AuthBody>, AuthError> {
    dbg!(&payload);
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        info!("login error empty msg");
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != cc.client_id || payload.client_secret != cc.client_secret {
        info!("incorrect msg");
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    println!("{}", &token);
    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

async fn authorize(
    Json(payload): Json<AuthPayload>,
    cc: Extension<ClientInfo>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials

    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        info!("login error empty msg");

        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != cc.client_id || payload.client_secret != cc.client_secret {
        info!("WrongCredentials");

        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    println!("{}", &token);
    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

///保护的展示抖音链接
async fn dy_info_list(dy_url: Json<Dyurl>, _cde: Hello) -> Json<Dylist> {
    let pd = list_elems(&dy_url.url).await;
    let mut newv = Vec::new();
    for i in pd {
        let dc: Vec<String> = i.split("@@").map(|x| x.to_string()).collect();
        // let d1 = format!("name:{}, url:{}", dc[0], dc[1]);
        let elp = Elp {
            name: dc[1].clone(),
            url: dc[0].clone(),
        };
        newv.push(elp);
    }
    let ppd = Dylist { elem: newv };

    // info!("{:?}", &ppd);z
    Json(ppd)
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequest<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    ///从header中将认证的token拿出来
    async fn from_request(parts: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

//定义一个结构体，引入从request获取参数的trait
#[derive(Debug, Serialize, Deserialize)]
struct Hello {
    // sub: String,
    // company: String,
    // exp: usize,
}

#[async_trait]
impl<S> FromRequest<S> for Hello
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request(parts: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header

        //拿出来远端
        println!("{:?}", parts.headers());
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Hello>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
