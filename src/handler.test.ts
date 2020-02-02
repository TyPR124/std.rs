import { rewriteUrl } from './handler';

[
  // Stable
  ['https://std.rs', 'https://doc.rust-lang.org/stable/std/'],
  [
    'https://std.rs/println',
    'https://doc.rust-lang.org/stable/std/?search=println',
  ],
  [
    'https://std.rs/print ln',
    'https://doc.rust-lang.org/stable/std/?search=print%20ln',
  ],
  // Nightly via path
  ['https://std.rs/n', 'https://doc.rust-lang.org/nightly/std/'],
  ['https://std.rs/n/', 'https://doc.rust-lang.org/nightly/std/'],
  [
    'https://std.rs/n/println',
    'https://doc.rust-lang.org/nightly/std/?search=println',
  ],
  [
    'https://std.rs/n/print ln',
    'https://doc.rust-lang.org/nightly/std/?search=print%20ln',
  ],
  // Nightly via subdomain
  ['https://n.std.rs', 'https://doc.rust-lang.org/nightly/std/'],
  ['https://n.std.rs/', 'https://doc.rust-lang.org/nightly/std/'],
  [
    'https://n.std.rs/println',
    'https://doc.rust-lang.org/nightly/std/?search=println',
  ],
  [
    'https://n.std.rs/print ln',
    'https://doc.rust-lang.org/nightly/std/?search=print%20ln',
  ],
  // Nightly via path and subdomain
  ['https://n.std.rs/n', 'https://doc.rust-lang.org/nightly/std/'],
  ['https://n.std.rs/n/', 'https://doc.rust-lang.org/nightly/std/'],
  [
    'https://n.std.rs/n/println',
    'https://doc.rust-lang.org/nightly/std/?search=println',
  ],
  [
    'https://n.std.rs/n/print ln',
    'https://doc.rust-lang.org/nightly/std/?search=print%20ln',
  ],
  // Can still search for ne
  ['https://std.rs/ne', 'https://doc.rust-lang.org/stable/std/?search=ne'],
  ['https://std.rs/n/ne', 'https://doc.rust-lang.org/nightly/std/?search=ne'],
  ['https://n.std.rs/ne', 'https://doc.rust-lang.org/nightly/std/?search=ne'],
  ['https://n.std.rs/n/ne', 'https://doc.rust-lang.org/nightly/std/?search=ne'],
].forEach(([from, to]) => {
  test(`rewrites ${from} to ${to}`, () => {
    expect(rewriteUrl(from)).toBe(to);
  });
});
