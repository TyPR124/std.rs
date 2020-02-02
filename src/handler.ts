export function rewriteUrl(url: string): string {
  const { hostname, pathname } = new URL(url);
  const isNightly = hostname.startsWith('n.') || pathname.startsWith('/n');

  const base = `https://doc.rust-lang.org/${
    isNightly ? 'nightly' : 'stable'
  }/std/`;
  const query = pathname.replace(/^(?:\/n)?\//, '');

  const finalUrl = `${base}${query ? `?search=${query}` : ''}`;

  return finalUrl;
}

export function handleRequest({ url }: Request): Response {
  const toUrl = rewriteUrl(url);
  return Response.redirect(toUrl, 301);
}
