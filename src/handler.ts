export function rewriteUrl(url: string): string {
  const { hostname, pathname } = new URL(url);
  const isNightly = hostname.startsWith('n.') || /^\/n(?:\/|$)/.test(pathname);

  const base = `https://doc.rust-lang.org/${
    isNightly ? 'nightly' : 'stable'
  }/std/`;
  // Remove any slashes from the start of the path, as well as /n/ (but not
  // words starting with /n such as /ne)
  const query = pathname.replace(/\/n(?:\/|$)/, '').replace(/^\/+/, '');

  const finalUrl = `${base}${query ? `?search=${query}` : ''}`;

  return finalUrl;
}

export function handleRequest({ url }: Request): Response {
  const toUrl = rewriteUrl(url);
  return Response.redirect(toUrl, 301);
}
