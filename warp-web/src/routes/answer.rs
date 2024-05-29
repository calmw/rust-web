use crate::{store, types};
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn add_answers(
    store: store::Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let answer = types::answer::Answer {
        id: types::answer::AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: types::question::QuestionId(params.get("questionId").unwrap().to_string()),
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
