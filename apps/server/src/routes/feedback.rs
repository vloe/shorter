use crate::{error::AppError, Ctx};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::env;
use typeshare::typeshare;

const MIN_MSG_LEN: usize = 10;
const MAX_MSG_LEN: usize = 256;

#[typeshare]
#[derive(Deserialize)]
pub struct FeedbackPayload {
    msg: String,
}

#[typeshare]
#[derive(Serialize)]
pub struct FeedbackRes {
    ok: bool,
}

impl FeedbackPayload {
    pub fn validate(&mut self) -> Result<&mut Self, AppError> {
        self.msg = self.msg.trim().to_string();
        if self.msg.len() < MIN_MSG_LEN {
            return Err(AppError::MsgTooShort(MIN_MSG_LEN));
        }
        if self.msg.len() > MAX_MSG_LEN {
            return Err(AppError::MsgTooLong(MAX_MSG_LEN));
        }
        Ok(self)
    }
}

pub async fn mount(
    State(ctx): State<Ctx>,
    Json(mut payload): Json<FeedbackPayload>,
) -> Result<Json<FeedbackRes>, AppError> {
    payload.validate()?;

    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| AppError::EnvVarNotSet("TELEGRAM_BOT_TOKEN".to_string()))?;
    let chat_id = env::var("TELEGRAM_CHAT_ID")
        .map_err(|_| AppError::EnvVarNotSet("TELEGRAM_CHAT_ID".to_string()))?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let params = [("chat_id", chat_id), ("text", payload.msg)];

    ctx.client
        .post(&url)
        .form(&params)
        .send()
        .await
        .map_err(|_| AppError::TelegramError())?;

    let res = FeedbackRes { ok: true };
    Ok(Json(res))
}
