use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, handle_errors::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(handle_errors::Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(handle_errors::Error::ParseError)?,
        });
    }
    Err(handle_errors::Error::MissingParameters)
}
