export default async function () {
  const {pathname} = location;
  const url = `http://localhost:${process.env.V_PORT}`;
  const res = await fetch(url);
  const html = await res.text();
  return html;
}
