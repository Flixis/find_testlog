const {
    exit
  } = window.__TAURI__.process;

async function error_ok(){
    await exit(-1);
}
$('#error-button').click(error_ok);
