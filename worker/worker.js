addEventListener('fetch', e => e.respondWith(handle(e.request)))

async function handle(request) {
  const { render_template } = wasm_bindgen;
  await wasm_bindgen(wasm)

  const url = new URL(request.url)
  const params =  new URLSearchParams(url.search)
  const template = params.get('template') || 'Hello, {{ name }}'
  const name = params.get('name') || 'world'

  const greeting = render_template(template, name)

  return new Response(greeting)
}
