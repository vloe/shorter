use crate::{error::AppError, utils::get_env, Ctx};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

const MSG_MIN: usize = 3;
const MSG_MAX: usize = 256;

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
        if self.msg.len() < MSG_MIN {
            return Err(AppError::TooShort("message", MSG_MIN));
        }
        if self.msg.len() > MSG_MAX {
            return Err(AppError::TooLong("message", MSG_MAX));
        }
        Ok(self)
    }
}

pub async fn mount(
    State(ctx): State<Ctx>,
    Json(mut payload): Json<FeedbackPayload>,
) -> Result<Json<FeedbackRes>, AppError> {
    payload.validate()?;

    let bot_token = get_env("TELEGRAM_BOT_TOKEN")?;
    let chat_id = get_env("TELEGRAM_CHAT_ID")?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let params = [("chat_id", chat_id), ("text", payload.msg)];

    ctx.reqwest
        .post(&url)
        .form(&params)
        .send()
        .await
        .map_err(|_| AppError::FuckUp())?;

    let res = FeedbackRes { ok: true };
    Ok(Json(res))
}
