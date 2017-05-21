import fetchRendering from '~/fetch-rendering';

(async () => {
  const html = await fetchRendering();
  document.getElementById('app').innerHTML = html;

  const res = await fetch(`http://localhost:8000/tasks`);
  const json = await res.json();

  const {default: renderReact} = await import('~/render-react');
  renderReact();
})();
