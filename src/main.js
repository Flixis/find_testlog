const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let yearWeekEl;
let greetMsgEl;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function getvaluefromtextbox() {
  greetMsgEl.textContent = await invoke("greet", { name: yearWeekEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  yearWeekEl = document.querySelector("#year_week");
  greetMsgEl = document.querySelector("#greet-msg");

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  // Assuming you have a form with id="year-week-form"
  document.querySelector("#year-week-form").addEventListener("submit", (e) => {
    e.preventDefault();
    getvaluefromtextbox();
  });
});
