use std::collections::HashMap;
use handle_errors::Error;

#[derive(Debug,Default)]
pub struct Pagination {
    pub limit: Option<i32>, // 可以是none或者一个数字，如果传递none，pg将默认忽略他，从而省去一些if语句
    pub offset: i32, //
}

pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, handle_errors::Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(params.get("limit").unwrap().parse::<i32>().map_err(Error::ParseError)?),
            offset: params.get("offset").unwrap().parse::<i32>().map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
