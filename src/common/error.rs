use actix_web::{HttpResponse, ResponseError, http::StatusCode, web::Json};
use serde::Serialize;
use std::fmt;
use tracing::error;

pub type ServiceResult<T> = Result<T, AppError>;
pub type ServiceResponse<T> = ServiceResult<Json<T>>;

#[track_caller]
pub fn unexpected<T, E: Into<anyhow::Error>>(e: E) -> ServiceResult<T> {
    let caller = std::panic::Location::caller();
    error!("An unexpected error has occurred at {caller}: {}", e.into());
    Err(AppError::Unexpected)
}

#[derive(Debug)]
pub enum AppError {
    Unexpected,
    Unauthorized,
    InternalServerError(&'static str),

    PlayerNotFound,
    PlayerMatchesNotFound,
    PlayerSteamIDInvalid,
    PlayerSteamDoesNotExist,

    MatchNotFound,
    MatchDetailNotFound,

    InvalidModel,
    UnevenTeams,

    AGDBInvalidSteamID,
    AGDBPlayerNotFound,
    AGDBPartialData,
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    #[track_caller]
    fn from(e: E) -> Self {
        unexpected::<(), E>(e).unwrap_err()
    }
}

impl AppError {
    pub const fn as_str(&self) -> &str {
        self.code()
    }

    pub const fn code(&self) -> &'static str {
        match self {
            AppError::Unexpected => "unexpected",
            AppError::Unauthorized => "unauthorized",
            AppError::InternalServerError(_) => "internal_server_error",

            AppError::PlayerNotFound => "player_not_found",
            AppError::PlayerMatchesNotFound => "player_matches_not_found",
            AppError::PlayerSteamIDInvalid => "player_steamid_invalid",
            AppError::PlayerSteamDoesNotExist => "player_steamid_does_not_exist",

            AppError::MatchNotFound => "match_not_found",
            AppError::MatchDetailNotFound => "match_detail_not_found",

            AppError::InvalidModel => "invalid_model",
            AppError::UnevenTeams => "uneven_teams",

            AppError::AGDBInvalidSteamID => "agdb_invalid_steamid",
            AppError::AGDBPlayerNotFound => "agdb_player_not_found",
            AppError::AGDBPartialData => "agdb_partial_data",
        }
    }

    pub const fn message(&self) -> &'static str {
        match self {
            AppError::Unexpected => "An unexpected error has occurred.",
            AppError::Unauthorized => "You are not authorized to perform this action.",
            AppError::InternalServerError(_) => "An internal server error has occurred.",

            AppError::PlayerNotFound => "The specified player was not found.",
            AppError::PlayerMatchesNotFound => "No matches found for the specified player.",
            AppError::PlayerSteamIDInvalid => "The provided Steam ID is invalid.",
            AppError::PlayerSteamDoesNotExist => "The provided Steam ID does not exist.",

            AppError::MatchNotFound => "The specified match was not found.",
            AppError::MatchDetailNotFound => "The specified match detail was not found.",

            AppError::InvalidModel => "Invalid model value. Valid values are 'blue' or 'red'.",
            AppError::UnevenTeams => "Team sizes do not match.",

            AppError::AGDBInvalidSteamID => "The provided Steam ID is invalid according to AGDB.",
            AppError::AGDBPlayerNotFound => "No player found in AGDB for the provided Steam ID.",
            AppError::AGDBPartialData => {
                "The provided Steam ID is valid and there is partial data in AGDB, but the API didn't return any data."
            }
        }
    }

    pub const fn http_status_code(&self) -> StatusCode {
        match self {
            AppError::PlayerSteamIDInvalid
            | AppError::InvalidModel
            | AppError::UnevenTeams
            | AppError::AGDBInvalidSteamID
            | AppError::AGDBPartialData => StatusCode::BAD_REQUEST,

            AppError::Unauthorized => StatusCode::UNAUTHORIZED,

            AppError::PlayerNotFound
            | AppError::PlayerMatchesNotFound
            | AppError::PlayerSteamDoesNotExist
            | AppError::MatchNotFound
            | AppError::MatchDetailNotFound
            | AppError::AGDBPlayerNotFound => StatusCode::NOT_FOUND,

            AppError::Unexpected | AppError::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub const fn response_parts(&self) -> (StatusCode, Json<ErrorResponse>) {
        let status = self.http_status_code();
        let response = ErrorResponse {
            code: self.code(),
            message: self.message(),
        };
        (status, Json(response))
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: &'static str,
    pub message: &'static str,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, body) = self.response_parts();
        HttpResponse::build(status).json(body)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            _ => write!(f, "{}", self.message()),
        }
    }
}
