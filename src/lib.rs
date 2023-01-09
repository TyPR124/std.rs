#![forbid(unsafe_code)]

use worker::{event, Context, Env, Request, Response, Result, Url};

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::redirect_with_status(redirect(&req.url()?), 301)
}

#[inline]
pub fn redirect(url: &Url) -> Url {
    let host = url.host_str().unwrap();

    let (nightly, query) = match url.path() {
        "/n/" | "/n" => (true, ""),
        path if path.starts_with("/n/") => (true, &path[3..]),
        path => (host.starts_with("n."), &path[1..]), // skip leading slash
    };

    Url::parse(&format!(
        "https://doc.rust-lang.org/{}/std/{}",
        if nightly { "nightly" } else { "stable" },
        if query.is_empty() {
            "".into()
        } else {
            format!("?search={}", query)
        }
    ))
    .unwrap()
}
