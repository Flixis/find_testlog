const { invoke } = window.__TAURI__.tauri;

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
  fakeProgressBar();

  // Loop through the data and create a row for each entry
  for (let i = 0; i < jsondata.datetime.length; i++) {
      const row = document.createElement('tr');
      const logLocation = jsondata.location[i].replace(/\\/g, '/'); // Replace backslashes with forward slashes
      row.innerHTML = `
          <td>${jsondata.datetime[i]}</td>
          <td>${jsondata.location[i]}</td>
          <td>${jsondata.serialnumber[i]}</td>
          <td>${jsondata.testenv[i]}</td>
          <td><button onclick='openLog("${logLocation}")'>Open Log</button></td>
          </tr>`;
      tableBody.appendChild(row);
  }
  
  // Update the results count
  const resultsCount = document.getElementById("results-count");
  // Update the results count when the search results are updated.
  function updateResultsCount() {
    resultsCount.textContent = document.getElementById("table-body").childElementCount;
  }
  // Update the results count when the page loads.
  updateResultsCount();
}

$('#search-button').click(execute_search);
// Add event listener to the search button to execute the search when the user presses enter
document.getElementById("search-form").addEventListener("keypress", function(event) {
  if (event.key === "Enter") {
    execute_search();
  }
});

//Parse the date from the datepicker to YYYY-WW
function FormatDateToYYYYWW(datepicker_id) {
 
  const input = document.getElementById(datepicker_id);
  const date = new Date(input.value);
  
  // Get the start date of the week (Monday)
  const Sunday = new Date(date);
  Sunday.setDate(Sunday.getDate());
  
  // Calculate the week number
  const firstDayOfYear = new Date(Sunday.getFullYear(), 0, 1);
  const weekNumber = Math.ceil((((Sunday - firstDayOfYear) / 86400000) + firstDayOfYear.getDay() + 1) / 7); //https://stackoverflow.com/questions/6117814/get-week-of-year-in-javascript-like-in-phpShout out google
  
  // Format the date
  const formattedDate = `${Sunday.getFullYear()}-W${String(weekNumber).padStart(2, '0')}`;
  
  //if the user doesn't pick a date in the datepicker then return an empty string
   if (formattedDate == "NaN-WNaN") { 
    return "";
  } 
  
  return formattedDate;  // Output: YYYY-WW
}

async function fakeProgressBar() {
  const loadingBar = document.querySelector('.loading-bar-inner');
  let width = 0;

  function frame() {
    if (width < 100) {
      width+=10;
      loadingBar.style.width = width + '%';
    } else {
      clearInterval(id);
    }
  }

  const id = setInterval(frame, 100); // Adjust the interval as needed
}




