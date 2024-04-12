const {
  invoke
} = window.__TAURI__.tauri;

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
  console.log(searchdata);

  const tableBody = document.getElementById('table-body');
  tableBody.innerHTML = ''; // Clear existing table data

  // Loop through the data and create a row for each entry
  for (let i = 0; i < Object.keys(searchdata).length; i++) {
      // Only print when it matches the following cases, or when it matches the test type
      if (test_type === "" || test_type.toUpperCase() === "ALL" || (searchdata[i].testtype || searchdata[i].name) === test_type.toUpperCase()) {
          const row = document.createElement('tr');
          const datetime = searchdata[i].datetime || searchdata[i].DateTime; // Use 'datetime' if available, otherwise use 'DateTime'
          const testtype = searchdata[i].operation_configuration || searchdata[i].operation; // Use 'testtype' if available, otherwise use 'Name'
          const clnt = searchdata[i].clnt || searchdata[i].Machine; // Use 'testtype' if available, otherwise use 'Name'
          const logLocation = searchdata[i].location.replace(/\\/g, '/'); // Replace backslashes with forward slashes
          const mode = searchdata[i].mode.trim().toLowerCase();
       
          row.innerHTML = `
      <td>${datetime}</td>
      <td>${testtype}</td>
      <td>${searchdata[i].release}</td>
      <td>${clnt}</td>
      <td>${searchdata[i].id}</td>
      <td><button onclick='openLog("${logLocation}")'>Open Log</button></td>
      </tr>`;

      const colors = {  
        PASS: '#1B9C85',
        FAIL: '#CC6852',
        SERVICE: '#CC6918bd' // Assuming 'service' is another status like PASS and FAIL
    };
    
    // Assuming 'i' is the current index in your process, and 'row' is the current row you're styling
    // Replace 'serialnumber' with the actual serial number you're checking
    let status = null; // Initialize status

    // Find the status for the given serial number in searchdata
    for (let item of searchdata) {
        if (item.hasOwnProperty(serialnumber)) {
            status = item[serialnumber]; // Get the status (PASS/FAIL)
            break; // Exit the loop once the serial number is found and status is assigned
        }
    }

    // Check if we found a status; if not, and mode is 'service', use SERVICE color
    if (status) {
        // Adjust the row color based on the status
        if (status === "PASS" || status === "PASSED") {
            row.style.backgroundColor = colors.PASS;
        } else if (status === "FAIL" || status === "FAILED") {
            row.style.backgroundColor = colors.FAIL;
        }
    } else if (mode === 'service') {
        // Directly use the 'SERVICE' color from the colors object
        row.style.backgroundColor = colors.SERVICE;
    }
    

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

async function updateProgressBar(updateamount) {
  const loadingBar = document.querySelector('.loading-bar-inner');
  if (loadingbarprogress + updateamount <= 100) {
      if (!loadingfinished && loadingbarprogress > 65) {
          updateamount = 0.25;
          if (loadingbarprogress > 80) {
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

function formatNumber(input, type) {
    // Remove all non-alphanumeric characters for a clean slate
    let cleanedInput = input.replace(/[^a-zA-Z0-9]/gi, '').toUpperCase();

    // Determine the pattern based on the input type
    let pattern;
    if (type === 'PN') { // Product Number: Insert dashes after every 4 digits
        pattern = [4, 4, 4]; // Example: 9999-1234-5678
    } else if (type === 'SN') { // Serial Number: XX-XX-YYY-000 pattern
        pattern = [2, 2, 3, 3];
    } else {
        return cleanedInput; // No formatting if type is unknown
    }

    let formatted = '';
    let index = 0;

    for (let i = 0; i < pattern.length; i++) {
        if (index >= cleanedInput.length) break;

        /* 
        Adds a dash before each new segment (except the first)
        and appends the segment of cleanedInput defined by the current pattern element,
        then updates the index to the next segment start position.
        */
        if (i > 0) formatted += '-';
        formatted += cleanedInput.substring(index, index + pattern[i]);
        index += pattern[i];
    }

    return formatted;
}

// Event listeners for input fields
document.getElementById('productnumber').addEventListener('input', function() {
    this.value = formatNumber(this.value, 'PN'); // Format as Product Number
    document.getElementById('formattedPN').textContent = `Formatted PN: ${this.value}`;
});

document.getElementById('serialnumber').addEventListener('input', function() {
    this.value = formatNumber(this.value, 'SN'); // Format as Serial Number
    document.getElementById('formattedSN').textContent = `Formatted SN: ${this.value}`;
});
