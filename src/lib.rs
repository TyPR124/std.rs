use url::Url;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub fn rewrite(url: String) -> String {
    let url = Url::parse(&url).unwrap();
    let host = url.host_str().unwrap();

    let (nightly, query) = match url.path() {
        "/n/" | "/n" => (true, ""),
        path if path.starts_with("/n/") => (true, &path[3..]),
        path => (host.starts_with("n."), &path[1..]), // skip leading slash
    };

    // create the redirect url
    format!(
        "https://doc.rust-lang.org/{}/std/{}",
        if nightly { "nightly" } else { "stable" },
        if query.is_empty() {
            "".into()
        } else {
            format!("?search={}", query)
        }
    )
}
