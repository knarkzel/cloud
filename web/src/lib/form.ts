export default function form(form_element) {

  async function handle_submit(event) {
    event.preventDefault();

    const response = await fetch(event.target.getAttribute('action'), {
      method: event.target.getAttribute('method'),
      body: new FormData(event.target)
    });

    const json = await response.json();

    event.target.dispatchEvent(
      new CustomEvent('form-response', { bubbles: true, detail: { json } })
    );
  }

  form_element.addEventListener('submit', handle_submit);
}
