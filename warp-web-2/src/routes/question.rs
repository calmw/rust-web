use crate::{store, types};
use std::collections::HashMap;
use tracing::info;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn update_questions(
    id: String,
    store: store::Store,
    question: types::question::Question,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .await
        .get_mut(&types::question::QuestionId(id))
    {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(handle_errors::Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn get_questions(
    params: HashMap<String, String>,
    store: store::Store,
    id: String,
) -> Result<impl Reply, Rejection> {
    info!("{} start querying question",id);
    if params.is_empty() {
        let pagination = types::pagination::extract_pagination(params)?;
        let res: Vec<types::question::Question> =
            store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<types::question::Question> =
            store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_questions(
    store: store::Store,
    question: types::question::Question,
) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn del_questions(id: String, store: store::Store) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .await
        .remove(&types::question::QuestionId(id))
    {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(handle_errors::Error::QuestionNotFound)),
    }
}
