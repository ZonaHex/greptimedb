// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::body::{boxed, Full};
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::Router;
use common_telemetry::debug;
use rust_embed::RustEmbed;
use snafu::ResultExt;

use crate::error::{BuildHttpResponseSnafu, Result};

#[derive(RustEmbed)]
#[folder = "dashboard/"]
pub struct Assets;

pub(crate) fn dashboard() -> Router {
    Router::new().fallback(static_handler)
}

#[axum_macros::debug_handler]
async fn static_handler(uri: Uri) -> Result<impl IntoResponse> {
    debug!("[dashboard] requesting: {}", uri.path());

    let mut path = uri.path().trim_start_matches('/');
    if path.is_empty() {
        path = "index.html";
    }

    match Assets::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(boxed(Full::from("404"))),
    }
    .context(BuildHttpResponseSnafu)
}
