const jsonData = `{
  "logs": [
    {
      "Date": "2023-09-01",
      "Time": "09:00 AM",
      "Location": "C:/path/to/log1",
      "SN": "SN123456",
      "Test Environment": "PTF",
      "Open Log": true
    },
    {
      "Date": "2023-09-02",
      "Time": "10:00 AM",
      "Location": "C:/path/to/log2",
      "SN": "SN789012",
      "Test Environment": "PTF",
      "Open Log": true
    }
  ]
}`;

const data = JSON.parse(jsonData);

function updateTable() {
  const pn = $('#pn').val();
  const sn = $('#sn').val();
  const yearWeek = $('#year_week').val();
  const testEnv = $('#test_env').val();

  const filteredLogs = data.logs.filter(log => {
    return (
      (!pn || log.SN.includes(pn)) &&
      (!sn || log.SN.includes(sn)) &&
      (!yearWeek || log.Date.includes(yearWeek)) &&
      (!testEnv || log['Test Environment'] === testEnv)
    );
  });

  const tableBody = $('#table-body');
  tableBody.empty();

  filteredLogs.forEach(log => {
    tableBody.append(`
      <tr>
        <td>${log.Date}</td>
        <td>${log.Time}</td>
        <td>${log.Location}</td>
        <td>${log.SN}</td>
        <td>${log['Test Environment']}</td>
        <td><button class="open-log-button" data-location="${log.Location}">Open Log</button></td>
      </tr>
    `);
  });
}

$('#search-button').click(updateTable);

// Add event listener to open log buttons
$('#table-body').on('click', '.open-log-button', function() {
  const location = $(this).data('location');
  // Implement logic to open the log file using location
  alert(`Opening log at location: ${location}`);
});

// Initial table load
updateTable();