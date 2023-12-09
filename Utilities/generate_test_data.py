import argparse
import random
import os
import datetime


def generate_random_semver():
    # Generate random major, minor, and patch versions
    major = random.randint(0, 9)
    minor = random.randint(0, 9)
    patch = random.randint(0, 9)
    
    # Generate a random prerelease string (5 random uppercase letters)
    prerelease = ''.join(random.choice('ABCDEFGHIJKLMNOPQRSTUVWXYZ') for _ in range(5))
    
    # Combine the components into a Semver string
    semver = f"v{major}.{minor}.{patch}-{prerelease}"
    
    return semver

def generate_log_file_inner(version, clnt, mode, pn, test_type_full, test_type):
 
    application_info = {
        'Name': 'Q\'s Test Framework',
        'Version': version,
        'Machine': clnt,
        'Mode': mode
    }

    random_id = random.randint(000000,999999)
    random_release = random.randint(000, 999)
    operation_config = f"{test_type} (id: {random_id}; Release R{random_release} (Latest))"
    
    test_info = {
        'PN': pn,
        'Operation': test_type_full,
        'Operation configuration': operation_config
    }

    text = "Application:\n"
    for key, value in application_info.items():
        text += f"- {key}: {value}\n"

    text += "Test:\n"
    for key, value in test_info.items():
        text += f"- {key}: {value}\n"

    return text

def generate_random_sn():
  """Generates a random SN in the format <2ints>-<2ints>-<3chars>-<3chars>."""
  sn = ""
  for i in range(2):
    sn += str(random.randint(0, 9))
  sn += "-"
  for i in range(2):
    sn += str(random.randint(0, 9))
  sn += "-"
  for i in range(3):
    sn += random.choice("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")
  sn += "-"
  for i in range(3):
    sn += random.choice("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")
  
  sn = "11-11-AAA-BBB" #overwrite for testing
  return sn

def generate_random_file_string(year, week):
  """Generates a random string in the format <date>_<time>_CLNT<randomint>_group_0_<random_sn>."""
  # Get the start and end dates of the specified week.
  start_date = datetime.datetime.strptime(f'{year}-W{week}-1', "%Y-W%U-%w")
  end_date = start_date + datetime.timedelta(days=6)
  
  # Generate a random date between the start and end dates.
  random_date = start_date + datetime.timedelta(seconds=random.randint(0, int((end_date-start_date).total_seconds())))
  random_date_str = random_date.strftime("%Y%m%d")
  random_time_str = random_date.strftime("%H%M%S")
  random_clnt = random.randint(1000, 9999)
  random_group = random.randint(0, 100)
  random_sn = generate_random_sn()
  # return f"{random_date_str}_{random_time_str}_CLNT{random_clnt}_group_0_{random_sn}.log"
  return random_date_str, random_time_str, f"CLNT{random_clnt}", f"group_{random_group}" ,random_sn

# Generates a random folder structure in the format <drive>:<folder>/<pn_formatted>/<year>-W<week_str>/<test_env>/<log_file_name>
def generate_random_folder_structure(drive, folder, pn_min, pn_max, year_min, year_max, week_min, week_max, test_suite_list, mode_type_list, test_type_list, test_type_list_full):
  """
  Generates a random folder structure in the format:

  <drive>:<folder>/<pn_formatted>/<year>-W<week_str>/<test_env>/<log_file_name>

  Args:
    drive: The drive letter to generate the folder structure on.
    folder: The folder name to generate the folder structure in.
    pn_min: The minimum PN value to use.
    pn_max: The maximum PN value to use.
    year_min: The minimum year value to use.
    year_max: The maximum year value to use.
    week_min: The minimum week value to use.
    week_max: The maximum week value to use.
    test_suite_list: A list of test environments to choose from.

  Returns:
    The path to the generated log file.
  """

  # Generate a random PN value.
  pn = random.randint(pn_min, pn_max)
  # Format the PN value as a string.
  pn_str = str(pn)
  # Pad the PN string to 12 characters.
  pn_formatted = f"{pn_str[:4]}-{pn_str[4:8]}-{pn_str[8:12]}"

  # Generate a random year value.
  year = random.randint(year_min, year_max)
  # Generate a random week value.
  week = random.randint(week_min, week_max)

  # Format the week value as a string and pad it to 2 characters.
  week_str = str(week).zfill(2)

  # Choose a random test environment.
  test_suite = random.choice(test_suite_list)
  mode_type = random.choice(mode_type_list)
  
  selected_test_type = random.choice(test_type_list)
  # Find the corresponding full name in test_type_list_full
  full_name_index = test_type_list.index(selected_test_type)
  selected_full_name = test_type_list_full[full_name_index]

  # Generate a random log file name.
  random_date_str, random_time_str, random_clnt, random_group ,random_sn  = generate_random_file_string(year, week)
  log_file_name = f"{random_date_str}_{random_time_str}_{random_clnt}_{random_group}_{random_sn}.log"

  # Construct the path to the log file.
  log_file_path = os.path.join(drive, folder, pn_formatted, f"{year}-W{week_str}", test_suite, log_file_name)

  # Create the directories if they don't already exist.
  os.makedirs(os.path.dirname(log_file_path), exist_ok=True)

  # Create the log file.
  with open(log_file_path, "w") as f:
    semver = generate_random_semver()
    text_in_file = generate_log_file_inner(semver, random_clnt, mode_type, pn_formatted,  selected_full_name, selected_test_type)
    f.write(text_in_file)

  # Return the path to the log file.
  return log_file_path

drive = "F:"
folder = "TestLogs"
pn_min = 999911112222
pn_max = 999911112222
year_min = 1998
year_max = 2023
week_min = 0 #0=1 
week_max = 51 #51=52
test_suite_list = ["PTF", "FT", "ET", "XT", "PI","AET", "ICT"]
test_type_list = ["FT", "ST", "FI", "ET", "DT","FT01", "XT01", "MAI"]
test_type_list_full = ["Functional test", "Safety test", "Functional inspection test", "Endurance test", "Development test", "Functional test", "Extra test", "MAI test"]
mode_type_list = ["Production", "Service", "Development"]


if __name__ == "__main__":
  parser = argparse.ArgumentParser()
  parser.add_argument("--count", type=int, default=1, help="The number of times to generate a random folder structure.")
  args = parser.parse_args()

  for i in range(args.count):
    log_file_path = generate_random_folder_structure(drive, folder, pn_min, pn_max, year_min, year_max, week_min, week_max, test_suite_list, mode_type_list, test_type_list, test_type_list_full)
    print(log_file_path)