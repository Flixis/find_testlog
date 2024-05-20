const {
  invoke
} = window.__TAURI__.tauri;

let loadingbarprogress = 0;
let updateInterval;
let loadingfinished = false;

async function execute_search() {
    const startTime = performance.now();

    initializeProgressBar();
    const searchData = await fetchSearchData();
    populateTableWithSearchData(searchData);
    finalizeProgressBar(startTime);
}

function initializeProgressBar() {
    loadingbarprogress = 0;
    let loadingfinished = false;
    updateInterval = setInterval(() => updateProgressBar(1), 250);
}

async function fetchSearchData() {
    const productNumber = document.getElementById('productnumber').value.trim().toUpperCase();
    const serialNumber = document.getElementById('serialnumber').value.trim().toUpperCase();
    const dateYYYYWW = FormatDateToYYYYWW('datepicker');
    const testType = document.getElementById('test_type').value.trim();

    return await invoke('parse_frontend_search_data', {
        productnumber: productNumber,
        serialnumber: serialNumber,
        dateyyyyww: dateYYYYWW,
        testtype: testType,
    });
}

function populateTableWithSearchData(searchData) {
    const tableBody = document.getElementById('table-body');
    tableBody.innerHTML = '';

    Object.values(searchData).forEach((data, index) => {
        if (shouldIncludeRow(data, index)) {
            const row = createTableRow(data);
            tableBody.appendChild(row);
        }
    });

    updateResultsCount();
}

function shouldIncludeRow(data, index) {
    const testType = document.getElementById('test_type').value.trim().toUpperCase();
    return testType === "" || testType === "ALL" || data.testtype === testType || data.name === testType;
}

function createTableRow(data) {
    console.log(data);
    const row = document.createElement('tr');
    let status = getStatus(data);

    let modeValue = data.Mode;
    const partial = data.partial;

    // Determine the mode symbol based on the mode value or status
    let modeSymbol;
    if(!status){

    }else{
        if (modeValue !== 'SERVICE' && status.includes('ABORT')) {
            modeSymbol = '⚠️';  // Set symbol if status contains 'ABORT'
        } else if (partial) {
            modeSymbol = '🔧';  // Partial test symbol
        } else {
            modeSymbol = '';  // Default, no symbol
        }
    }
        
    row.innerHTML = `
        <td>${data.datetime || data.DateTime}</td>
        <td>
            <span class="alert-indicator" title="Mode: ${data.Mode}">
                ${modeSymbol}
            </span>
            ${data.Operation_configuration || data.operation}
        </td>
        <td>${data.Release}</td>
        <td>${data.clnt || data.Machine}</td>
        <td>${data.id}</td>
        <td><button onclick='openLog("${data.location.replace(/\\/g, '/')}")'>Open Log</button></td>
    `;
    styleRowBasedOnStatus(row, data); 
    return row;
}



function styleRowBasedOnStatus(row, data) {
    const statusColors = {
        PASS: '#1B9C85',
        FAIL: '#CC6852',
        SERVICE: '#CC6918bd'
    };

    let status = getStatus(data);

    if (status) {
        row.style.backgroundColor = statusColors[status] || null;
    }
}

function getStatus(data) {

    // Check first for service, its more relevant at first than pass or fail
    const mode = data.Mode;
    if (mode == "Service") {
        return 'SERVICE';
    }

    // Checking for PASS/FAIL status directly mentioned in data
    if (data.hasOwnProperty('PASS_FAIL_STATUS')) {
        const passFailStatus = data.PASS_FAIL_STATUS.trim().toUpperCase();
        if (passFailStatus.includes("PASS")) {
            return 'PASS';
        } else if (passFailStatus.includes("FAIL")) {
            return 'FAIL';
        }
        else if (passFailStatus.includes("ABORT")) {
            return 'ABORT';
        }
    }



    // Check for a status key that matches a specific serial number in the data
    const statusKey = document.getElementById('serialnumber').value.trim();
    if (statusKey && data.hasOwnProperty(statusKey)) {
        return data[statusKey].trim().toUpperCase();
    }

    // No relevant status found
    return null;
}




function updateResultsCount() {
    const resultsCount = document.getElementById("results-count");
    resultsCount.textContent = document.getElementById("table-body").childElementCount.toString();
}

function finalizeProgressBar(startTime) {
    clearInterval(updateInterval);
    const endTime = performance.now();
    const executionTime = endTime - startTime;
    const seconds = Math.floor(executionTime / 1000);
    const milliseconds = executionTime % 1000;
    if (seconds < 3){
        document.getElementById("results-box-time").innerText = `Time to results: ${seconds} seconds and ${milliseconds.toFixed(3)} milliseconds 🔥`;
    }else{
        document.getElementById("results-box-time").innerText = `Time to results: ${seconds} seconds and ${milliseconds.toFixed(3)} milliseconds`;
    }
    document.querySelector('.loading-bar-inner').style.width = '100%';
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

// Automatic hyphenation
document.getElementById('productnumber').addEventListener('input', function() {
    this.value = formatNumber(this.value, 'PN'); // Format as Product Number
    document.getElementById('formattedPN').textContent = `Formatted PN: ${this.value}`;
});

document.getElementById('serialnumber').addEventListener('input', function() {
    this.value = formatNumber(this.value, 'SN'); // Format as Serial Number
    document.getElementById('formattedSN').textContent = `Formatted SN: ${this.value}`;
});
