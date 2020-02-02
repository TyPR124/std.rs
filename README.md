# std.rs

This repository contains the source code behind the redirect worker running
[std.rs][s]. Currently it supports the following redirects:

| From                       | To                                                      |
| :------------------------- | :------------------------------------------------------ |
| `https://std.rs`           | `https://doc.rust-lang.org/stable/std/`                 |
| `https://n.std.rs`         | `https://doc.rust-lang.org/nightly/std/`                |
| `https://std.rs/n`         | `https://doc.rust-lang.org/nightly/std/`                |
| `https://std.rs/println`   | `https://doc.rust-lang.org/stable/std/?search=println`  |
| `https://n.std.rs/println` | `https://doc.rust-lang.org/nightly/std/?search=println` |
| `https://std.rs/n/println` | `https://doc.rust-lang.org/nightly/std/?search=println` |

## Contributing

Contributions via pull request are welcome! Just make sure you add tests for
any functionality you add and make sure the previous tests still pass.

## License

**MIT**

[s]: https://std.rs/
