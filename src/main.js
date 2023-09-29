const { invoke } = window.__TAURI__.tauri;


async function test_environment() {

    const pn = document.getElementById("pn").value;
    const sn = document.getElementById("sn").value;
    const yearweek = document.getElementById("yearweek").value;
    const testenv = document.getElementById("test_env").value;


    var jsondata = await invoke("testing_environment",{
        pn: pn,
        sn: sn,
        yearweek: yearweek,
        testenv: testenv,
    });
    console.log(jsondata);
}


$('#search-button').click(test_environment);
