const { invoke } = window.__TAURI__.tauri;


//Parse the date from the datepicker to YYYY-WW
function FormatDateToYYYYWW(datepicker_id) {
 
  const input = document.getElementById(datepicker_id);
  const date = new Date(input.value);
  
  // Get the start date of the week (Monday)
  const monday = new Date(date);
  monday.setDate(monday.getDate());
  
  // Calculate the week number
  const firstDayOfYear = new Date(monday.getFullYear(), 0, 1);
  const weekNumber = Math.ceil((((monday - firstDayOfYear) / 86400000) + firstDayOfYear.getDay() + 1) / 7); //https://stackoverflow.com/questions/6117814/get-week-of-year-in-javascript-like-in-phpShout out google
  
  // Format the date
  const formattedDate = `${monday.getFullYear()}-W${String(weekNumber).padStart(2, '0')}`;
  
  //if the user doesn't pick a date in the datepicker then return an empty string
   if (formattedDate == "NaN-WNaN") { 
    return "";
  } 
  
  return formattedDate;  // Output: YYYY-WW
}


async function execute_search() {
  const productnumber = document.getElementById('productnumber').value;
  const serialnumber = document.getElementById('serialnumber').value;
  const date_yyyyww = FormatDateToYYYYWW('datepicker');
  const testenv = document.getElementById("test_env").value;

  var jsondata = await invoke('parse_frontend_search_data', {
      productnumber: productnumber,
      serialnumber: serialnumber,
      dateyyyyww: date_yyyyww,
      testenv: testenv,
  });

  const tableBody = document.getElementById('table-body');
  tableBody.innerHTML = ''; // Clear existing table data

  // Loop through the data and create a row for each entry
  for (let i = 0; i < jsondata.datetime.length; i++) {
      const row = document.createElement('tr');
      row.innerHTML = `
          <td>${jsondata.datetime[i]}</td>
          <td>${jsondata.location[i]}</td>
          <td>${jsondata.serialnumber[i]}</td>
          <td>${jsondata.testenv[i]}</td>
          <td><button onclick="openLog('${jsondata.location[i]}')">Open Log</button></td>
          </tr>`;
      tableBody.appendChild(row);
  }
  
}

$('#search-button').click(execute_search);


