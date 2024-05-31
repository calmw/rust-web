mod routes;
mod store;
mod types;

use warp::http::Method;
use warp::{Filter};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "practical_rust_book=info,warp=error".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter) // 使用上面的过滤器来决定记录那些追踪
        .with_span_events(FmtSpan::CLOSE) // 在每个span关闭时记录event，可以用来计算路由执行时间
        .init();

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        })
        );

    // 以json方式提交
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json()) // 添加json过滤器
        .and_then(routes::question::add_questions);

    // 以表单方式提交
    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form()) // 添加表单过滤器
        .and_then(routes::answer::add_answers);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>()) // 添加一个字符串参数，由此过滤器将会被/questions/1234这样的路径出发
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json()) // 提取json正文，该正文稍后会被添加到参数中
        .and_then(routes::question::update_questions); // 以存储和json正文作为参数调用update_question

    let del_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>()) // 添加一个字符串参数，由此过滤器将会被/questions/1234这样的路径出发
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::del_questions); // 以存储和json正文作为参数调用update_question

    let routes = get_question
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(del_question)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
