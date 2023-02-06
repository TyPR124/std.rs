use std_rs_redirect_worker::redirect;
use worker::Url;

fn assert_redirect(input: &str, output: &str) {
    assert_eq!(redirect(&Url::parse(input).unwrap()).to_string(), output)
}

#[test]
fn stable() {
    assert_redirect("https://std.rs", "https://doc.rust-lang.org/stable/std/")
}
#[test]
fn stable_search() {
    assert_redirect(
        "https://std.rs/println",
        "https://doc.rust-lang.org/stable/std/?search=println",
    )
}
#[test]
fn nightly_search_space() {
    assert_redirect(
        "https://std.rs/print ln",
        "https://doc.rust-lang.org/stable/std/?search=print%20ln",
    )
}
#[test]
fn nightly_path() {
    assert_redirect("https://std.rs/n", "https://doc.rust-lang.org/nightly/std/")
}
#[test]
fn nightly_path_slash() {
    assert_redirect(
        "https://std.rs/n/",
        "https://doc.rust-lang.org/nightly/std/",
    )
}
#[test]
fn nightly_path_search() {
    assert_redirect(
        "https://std.rs/n/println",
        "https://doc.rust-lang.org/nightly/std/?search=println",
    )
}
#[test]
fn nightly_path_search_space() {
    assert_redirect(
        "https://std.rs/n/print ln",
        "https://doc.rust-lang.org/nightly/std/?search=print%20ln",
    )
}
#[test]
fn nightly_domain() {
    assert_redirect("https://n.std.rs", "https://doc.rust-lang.org/nightly/std/")
}
#[test]
fn nightly_domain_slash() {
    assert_redirect(
        "https://n.std.rs/",
        "https://doc.rust-lang.org/nightly/std/",
    )
}
#[test]
fn nightly_domain_search() {
    assert_redirect(
        "https://n.std.rs/println",
        "https://doc.rust-lang.org/nightly/std/?search=println",
    )
}
#[test]
fn nightly_domain_search_space() {
    assert_redirect(
        "https://n.std.rs/print ln",
        "https://doc.rust-lang.org/nightly/std/?search=print%20ln",
    )
}
#[test]
fn nightly_both() {
    assert_redirect(
        "https://n.std.rs/n",
        "https://doc.rust-lang.org/nightly/std/",
    )
}
#[test]
fn nightly_both_slash() {
    assert_redirect(
        "https://n.std.rs/n/",
        "https://doc.rust-lang.org/nightly/std/",
    )
}
#[test]
fn nightly_both_search() {
    assert_redirect(
        "https://n.std.rs/n/println",
        "https://doc.rust-lang.org/nightly/std/?search=println",
    )
}
#[test]
fn nightly_both_search_space() {
    assert_redirect(
        "https://n.std.rs/n/print ln",
        "https://doc.rust-lang.org/nightly/std/?search=print%20ln",
    )
}
#[test]
fn stable_ne() {
    assert_redirect(
        "https://std.rs/ne",
        "https://doc.rust-lang.org/stable/std/?search=ne",
    )
}
#[test]
fn nightly_path_ne() {
    assert_redirect(
        "https://std.rs/n/ne",
        "https://doc.rust-lang.org/nightly/std/?search=ne",
    )
}
#[test]
fn nightly_domain_ne() {
    assert_redirect(
        "https://n.std.rs/ne",
        "https://doc.rust-lang.org/nightly/std/?search=ne",
    )
}
#[test]
fn nightly_both_ne() {
    assert_redirect(
        "https://n.std.rs/n/ne",
        "https://doc.rust-lang.org/nightly/std/?search=ne",
    )
}
