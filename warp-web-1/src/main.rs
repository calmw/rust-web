mod routes;
mod store;
mod types;

use warp::http::Method;
use warp::Filter;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::error!("this is an error!");
    log::info!("this is info!");
    log::warn!("this is a warning!");
    let log = warp::log::custom(|info| {
        // eprintln!( // 不会输出json格式
        log::info!(
            "{} {} {} {:?} from with {:?} {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers(),
        );
    });

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::question::get_questions);

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
        .and(warp::path::param::<String>()) // 添加一个字符串参数，由此过滤器将会被/questions/1234这样的路径出发
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json()) // 提取json正文，该正文稍后会被添加到参数中
        .and_then(routes::question::update_questions); // 以存储和json正文作为参数调用update_question

    let del_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>()) // 添加一个字符串参数，由此过滤器将会被/questions/1234这样的路径出发
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::del_questions); // 以存储和json正文作为参数调用update_question

    let routes = get_question
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(del_question)
        .with(cors)
        .with(log)
        .recover(handle_errors::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
