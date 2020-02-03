addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
    const { rewrite } = wasm_bindgen;
    await wasm_bindgen(wasm)
    return Response.redirect(rewrite(request.url), 301)
}
