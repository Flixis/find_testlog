const { invoke } = window.__TAURI__.tauri;

async function loadDataAndUpdateTable() {
  // Get the form values
  const pn = $('#pn').val();
  const sn = $('#sn').val();
  const yearWeek = $('#year_week').val();
  const testEnv = $('#test_env').val();

  // Send the form values to Rust and receive the JSON data back
  const jsonData = await invoke('data_to_frontend', {
    pn,
    sn,
    yearWeek,
    testEnv,
  });

  // Error handling
  if (jsonData === null || typeof jsonData !== 'string') {
    alert('Error getting log file paths.');
    return;
  }

  // Parse the JSON data
  const data = JSON.parse(jsonData);

  // Check data structure
  if (!data || !Array.isArray(data.logs)) {
    console.log(data);  // Debugging line to inspect data structure
    alert('Error: Unexpected data structure.');
    return;
  }

  updateTable(data);
}

function updateTable(data) {
  // Filter the logs based on the form values
  const filteredLogs = data.logs.filter(log => {
    const pn = $('#pn').val();
    const sn = $('#sn').val();
    const yearWeek = $('#year_week').val();
    const testEnv = $('#test_env').val();
    return (
      (!pn || log.SN.includes(pn)) &&
      (!sn || log.SN.includes(sn)) &&
      (!yearWeek || log.Date.includes(yearWeek)) &&
      (!testEnv || log['Test Environment'] === testEnv)
    );
  });

  // Update the table with the filtered logs
  const tableBody = $('#table-body');
  tableBody.empty();
  
  filteredLogs.forEach(log => {
    tableBody.append(`
      <tr>
        <td>${log.Date}</td>
        <td>${log.Time}</td>
        <td>${log.Location}</td>
        <td>${log.SN}</td>
        <td><button class="open-log-button" data-location="${log.Location}">Open Log</button></td>
      </tr>
    `);
  });
}

$('#search-button').click(loadDataAndUpdateTable);

// Add event listener to open log buttons
$('#table-body').on('click', '.open-log-button', function() {
  const location = $(this).data('location');
  // Implement logic to open the log file using location
  alert(`Opening log at location: ${location}`);
});

// Initial table load
loadDataAndUpdateTable();
