use crate::{store, types};
use std::collections::HashMap;
use handle_errors::Error;
use tracing::{instrument, info};
use warp::http::StatusCode;
use warp::{Rejection, Reply};
use warp::path::param;
use crate::types::question::Question;
use crate::types::pagination::{extract_pagination, Pagination};

pub async fn update_questions(
    id: i32,
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

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    info!(" start querying question");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = extract_pagination(param)?;
    }
    let res: Vec<Question> = match store.get_questions(pagination.limit, pagination.offset).await {
        Ok(res) => res,
        Err(e) => {
            Err(
                warp::reject::custom(Error::DatabaseQueryError(e))
            )
        }
    };
    Ok(warp::reply::json(&res))
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

pub async fn del_questions(id: i32, store: store::Store) -> Result<impl Reply, Rejection> {
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
