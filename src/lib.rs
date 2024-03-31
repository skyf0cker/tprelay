mod telegraph;

use std::u32;

use http::StatusCode;
use worker::*;

const DEFAULT_PAGE_SIZE: u32 = 10;
const FIRST_PAGE: u32 = 0;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let token = match env.var("TELEGRAPH_ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return Response::error(
                "env var TELEGRAPH_ACCESS_TOKEN not found",
                StatusCode::INTERNAL_SERVER_ERROR.into(),
            )
        }
    };

    let url = req.url()?;
    let query_pairs = url.query_pairs();

    let mut page_num = FIRST_PAGE;
    let mut page_size = DEFAULT_PAGE_SIZE;

    for (key, value) in query_pairs {
        if key == "page_num" {
            page_num = match value.parse::<u32>() {
                Ok(page_num) => page_num,
                Err(_) => {
                    return Response::error("invalid page_num", StatusCode::BAD_REQUEST.into())
                }
            };
        } else if key == "page_size" {
            page_size = match value.parse::<u32>() {
                Ok(page_size) => page_size,
                Err(_) => {
                    return Response::error("invalid page_size", StatusCode::BAD_REQUEST.into())
                }
            };
        }
    }

    let client = telegraph::Client::new(&token.to_string());
    return match client.get_page_list(page_num, page_size).await {
        Ok(page_list) => Response::from_json(&page_list),
        Err(err) => Response::error(
            format!("get telegraph page list failed, error: {err}"),
            StatusCode::INTERNAL_SERVER_ERROR.into(),
        ),
    };
}
