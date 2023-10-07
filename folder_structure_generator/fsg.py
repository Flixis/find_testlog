import argparse
import random
import os
import datetime

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
  
  sn = "99-11-AAA-BBB" #overwrite for testing
  return sn

def generate_random_string(year, week):
  """Generates a random string in the format <date>_<time>_CLNT<randomint>_group_0_<random_sn>."""
  # Get the start and end dates of the specified week.
  start_date = datetime.datetime.strptime(f'{year}-W{week}-1', "%Y-W%U-%w")
  end_date = start_date + datetime.timedelta(days=6)
  
  # Generate a random date between the start and end dates.
  random_date = start_date + datetime.timedelta(seconds=random.randint(0, int((end_date-start_date).total_seconds())))
  random_date_str = random_date.strftime("%Y%m%d")
  random_time_str = random_date.strftime("%H%M%S")
  random_int = random.randint(1000, 9999)
  random_sn = generate_random_sn()  # Ensure you have this function defined elsewhere
  return f"{random_date_str}_{random_time_str}_CLNT{random_int}_group_0_{random_sn}.log"

# Generates a random folder structure in the format <drive>:<folder>/<pn_formatted>/<year>-W<week_str>/<test_env>/<log_file_name>
def generate_random_folder_structure(drive, folder, pn_min, pn_max, year_min, year_max, week_min, week_max, test_env_list):
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
    test_env_list: A list of test environments to choose from.

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
  test_env = random.choice(test_env_list)

  # Generate a random log file name.
  log_file_name = generate_random_string(year, week)

  # Construct the path to the log file.
  log_file_path = os.path.join(drive, folder, pn_formatted, f"{year}-W{week_str}", test_env, log_file_name)

  # Create the directories if they don't already exist.
  os.makedirs(os.path.dirname(log_file_path), exist_ok=True)

  # Create the log file.
  with open(log_file_path, "w") as f:
    f.write("This is a test log file." + log_file_name)

  # Return the path to the log file.
  return log_file_path

drive = "D:"
folder = "TestLogs"
pn_min = 999911112222
pn_max = 999911112222
year_min = 1998
year_max = 2023
week_min = 0 #0=1 
week_max = 51 #51=52
test_env_list = ["PTF", "FT", "ET", "XT", "PI","AET", "ICT"]

if __name__ == "__main__":
  parser = argparse.ArgumentParser()
  parser.add_argument("--count", type=int, default=1, help="The number of times to generate a random folder structure.")
  args = parser.parse_args()

  for i in range(args.count):
    log_file_path = generate_random_folder_structure(drive, folder, pn_min, pn_max, year_min, year_max, week_min, week_max, test_env_list)
    print(log_file_path)