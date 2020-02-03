use crate::rewrite;

#[test]
fn stable() {
    assert_eq!(
        rewrite("https://std.rs".into()),
        "https://doc.rust-lang.org/stable/std/"
    )
}
#[test]
fn stable_search() {
    assert_eq!(
        rewrite("https://std.rs/println".into()),
        "https://doc.rust-lang.org/stable/std/?search=println"
    )
}
#[test]
fn nightly_search_space() {
    assert_eq!(
        rewrite("https://std.rs/print ln".into()),
        "https://doc.rust-lang.org/stable/std/?search=print%20ln"
    )
}
#[test]
fn nightly_path() {
    assert_eq!(
        rewrite("https://std.rs/n".into()),
        "https://doc.rust-lang.org/nightly/std/"
    )
}
#[test]
fn nightly_path_slash() {
    assert_eq!(
        rewrite("https://std.rs/n/".into()),
        "https://doc.rust-lang.org/nightly/std/"
    )
}
#[test]
fn nightly_path_search() {
    assert_eq!(
        rewrite("https://std.rs/n/println".into()),
        "https://doc.rust-lang.org/nightly/std/?search=println"
    )
}
#[test]
fn nightly_path_search_space() {
    assert_eq!(
        rewrite("https://std.rs/n/print ln".into()),
        "https://doc.rust-lang.org/nightly/std/?search=print%20ln"
    )
}
#[test]
fn nightly_domain() {
    assert_eq!(
        rewrite("https://n.std.rs".into()),
        "https://doc.rust-lang.org/nightly/std/"
    )
}
#[test]
fn nightly_domain_slash() {
    assert_eq!(
        rewrite("https://n.std.rs/".into()),
        "https://doc.rust-lang.org/nightly/std/"
    )
}
#[test]
fn nightly_domain_search() {
    assert_eq!(
        rewrite("https://n.std.rs/println".into()),
        "https://doc.rust-lang.org/nightly/std/?search=println"
    )
}
#[test]
fn nightly_domain_search_space() {
    assert_eq!(
        rewrite("https://n.std.rs/print ln".into()),
        "https://doc.rust-lang.org/nightly/std/?search=print%20ln"
    )
}
#[test]
fn nightly_both() {
    assert_eq!(
        rewrite("https://n.std.rs/n".into()),
        "https://doc.rust-lang.org/nightly/std/"
    )
}
#[test]
fn nightly_both_slash() {
    assert_eq!(
        rewrite("https://n.std.rs/n/".into()),
        "https://doc.rust-lang.org/nightly/std/"
    )
}
#[test]
fn nightly_both_search() {
    assert_eq!(
        rewrite("https://n.std.rs/n/println".into()),
        "https://doc.rust-lang.org/nightly/std/?search=println"
    )
}
#[test]
fn nightly_both_search_space() {
    assert_eq!(
        rewrite("https://n.std.rs/n/print ln".into()),
        "https://doc.rust-lang.org/nightly/std/?search=print%20ln"
    )
}
#[test]
fn stable_ne() {
    assert_eq!(
        rewrite("https://std.rs/ne".into()),
        "https://doc.rust-lang.org/stable/std/?search=ne"
    )
}
#[test]
fn nightly_path_ne() {
    assert_eq!(
        rewrite("https://std.rs/n/ne".into()),
        "https://doc.rust-lang.org/nightly/std/?search=ne"
    )
}
#[test]
fn nightly_domain_ne() {
    assert_eq!(
        rewrite("https://n.std.rs/ne".into()),
        "https://doc.rust-lang.org/nightly/std/?search=ne"
    )
}
#[test]
fn nightly_both_ne() {
    assert_eq!(
        rewrite("https://n.std.rs/n/ne".into()),
        "https://doc.rust-lang.org/nightly/std/?search=ne"
    )
}
