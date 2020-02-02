export function handleRequest(request: Request): Response {
  const { hostname, pathname } = new URL(request.url);
  const isNightly = hostname.startsWith('n.') || pathname.startsWith('/n');

  const base = `https://doc.rust-lang.org/${
    isNightly ? 'nightly' : 'stable'
  }/std/`;
  const query = pathname.replace(/^(?:\/n)?\//, '');
  const finalUrl = `${base}${
    query ? `?search=${encodeURIComponent(query)}` : ''
  }`;

  return Response.redirect(finalUrl, 301);
}
