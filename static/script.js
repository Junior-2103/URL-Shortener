document.getElementById('url-form').addEventListener('submit', (e) => {
  e.preventDefault();

  const link = document.getElementById('link')
  const form = e.target;
  const data = {
    url: form.url.value
  };

  fetch('/shorten', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  })
  .then(res => res.json())
  .then(data => {
    let baseUrl = window.location.origin;

    link.innerHTML = `Acesse em: <a href="${baseUrl}/link/${data['link_uuid']}">${baseUrl}/link/${data['link_uuid']}</a>`
  })
  .catch(err => console.error('Erro:', err));
});   
