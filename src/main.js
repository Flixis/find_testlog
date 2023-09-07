const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;

async function getvaluefromtextbox() {
  greetMsgEl = document.querySelector("#greet-msg");
  greetMsgEl.textContent = await invoke("greet", { name: pn.value });
  greetMsgEl.textContent = await invoke("greet", { name: sn.value });
  greetMsgEl.textContent = await invoke("greet", { name: yearWeek.value });
  greetMsgEl.textContent = await invoke("greet", { name: testEnv.value });
}

window.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector("#search-button-form");

  form.addEventListener("submit", (e) => {
    // Prevent the default form submission behavior
    e.preventDefault();

    // Collect the input values
    const pn = document.getElementById("pn").value;
    const sn = document.getElementById("sn").value;
    const yearWeek = document.getElementById("year_week").value;
    const testEnv = document.getElementById("test_env").value;

    // Create an object with the collected data
    const searchData = {
      pn: pn,
      sn: sn,
      yearWeek: yearWeek,
      testEnv: testEnv,
    };

    // Now you can do something with the searchData object, such as sending it to the server or performing a search operation.
    getvaluefromtextbox()

    
    // For example, if you want to log the data to the console:
    console.log(searchData);
  });
});

