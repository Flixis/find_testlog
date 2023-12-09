const { invoke } = window.__TAURI__.tauri;

let loadingbarprogress = 0;
let updateInterval;
let loadingfinished = false;

async function execute_search() {

  //start counting time for execution
  const startTime = performance.now();

  //reset the progress bar
  loadingbarprogress = 0;
  let loadingfinished = false;
  updateInterval = setInterval(() => {
    updateProgressBar(1); // Update amount of progress
  }, 250); // Adjust the interval as needed
  
  //grab the important elements
  const productnumber = document.getElementById('productnumber').value.trim();
  const serialnumber = document.getElementById('serialnumber').value.trim();
  const date_yyyyww = FormatDateToYYYYWW('datepicker');
  const test_type = document.getElementById('test_type').value.trim();
  
  var searchdata = await invoke('parse_frontend_search_data', {
      productnumber: productnumber,
      serialnumber: serialnumber,
      dateyyyyww: date_yyyyww,
      testtype: test_type,
  });

  const tableBody = document.getElementById('table-body');
  tableBody.innerHTML = ''; // Clear existing table data

// Loop through the data and create a row for each entry
for (let i = 0; i < Object.keys(searchdata).length; i++) {
  // Only print when it matches the following cases, or when it matches the test type
  if (test_type === "" || test_type.toUpperCase() === "ALL" || (searchdata[i].testtype || searchdata[i].name) === test_type.toUpperCase()) {
      const row = document.createElement('tr');
      const datetime = searchdata[i].datetime || searchdata[i].DateTime; // Use 'datetime' if available, otherwise use 'DateTime'
      const testtype = searchdata[i].testtype || searchdata[i].Name; // Use 'testtype' if available, otherwise use 'Name'
      row.innerHTML = `
        <td>${datetime}</td>
        <td>${testtype}</td>
        <td>${searchdata[i].release}</td>
        <td>${searchdata[i].clnt}</td>
        <td>${searchdata[i].id}</td>
        <td><button onclick='openLog("${searchdata[i].location}")'>Open Log</button></td>
        </tr>`;
      tableBody.appendChild(row);
  }
}


  // Update the results count
  const resultsCount = document.getElementById("results-count");
  // Update the results count when the search results are updated.
  function updateResultsCount() {
      resultsCount.textContent = document.getElementById("table-body").childElementCount;
  }
  // Update the results count when the page loads.
  updateResultsCount();

  // Stop the interval and finalize the progress bar
  clearInterval(updateInterval);
  loadingfinished = true;
  const loadingBar = document.querySelector('.loading-bar-inner');
  loadingBar.style.width = '100%';

  // calculate the total time for execution
  const endTime = performance.now();
  const executionTime = endTime - startTime;
  const seconds = Math.floor(executionTime / 1000);
  const milliseconds = executionTime % 1000;
  document.getElementById("results-box-time").innerText = `Time to results: ${seconds} seconds and ${milliseconds.toFixed(3)} milliseconds`;
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

  return formattedDate; // Output: YYYY-WW
}

//I call this function many times to update the progressbar, for some reason I can't get the async to well... Async.
async function updateProgressBar(updateamount) {
  const loadingBar = document.querySelector('.loading-bar-inner');
  if (loadingbarprogress + updateamount <= 100) {
      if (!loadingfinished && loadingbarprogress > 65) {
        updateamount = 0.25;
        if(loadingbarprogress > 80){
          updateamount = 0.05;
        }
      }
      loadingbarprogress += updateamount;
      loadingBar.style.width = loadingbarprogress + '%';
    } else {
      loadingbarprogress = 100;
      loadingBar.style.width = '100%';
  }
}

