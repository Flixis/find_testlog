const {
  invoke
} = window.__TAURI__.tauri;

async function error_ok(){
  console.log("here");
  await invoke('kill_app'); //if the err dialog pops we should kill the app after user acknowledgement.
}
$('#error-button-ok').click(error_ok);
