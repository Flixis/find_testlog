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
  return sn

def generate_random_string():
  """Generates a random string in the format <date>_<time>_CLNT<randomint>_group_0_<random_sn>."""
  date = datetime.datetime.today().strftime("%Y%m%d")
  time = datetime.datetime.today().strftime("%H%M%S")
  random_int = random.randint(1000, 9999)
  random_sn = generate_random_sn()
  return f"{date}_{time}_CLNT{random_int}_group_0_{random_sn}.log"

def generate_random_folder_structure(drive, folder, pn_min, pn_max, year_min, year_max, week_min, week_max, test_env_list):
    pn = random.randint(pn_min, pn_max)
    pn_str = str(pn)
    pn_formatted = f"{pn_str[:4]}-{pn_str[4:8]}-{pn_str[8:12]}"
    
    year = random.randint(year_min, year_max)
    week = random.randint(week_min, week_max)
    
    week_str = str(week).zfill(2)
    
    test_env = random.choice(test_env_list)
    
    log_file_name = generate_random_string() #replace this with "20231006_194142_CLNT2228_group_0_41-82-SBV-LBK" or some shit if you wanna have the same sn in different folders.
    log_file_path = os.path.join(drive, folder, pn_formatted, f"{year}-W{week_str}", test_env, log_file_name)
    os.makedirs(os.path.dirname(log_file_path), exist_ok=True)
        
    with open(log_file_path, "w") as f:
        f.write("This is a test log file." + log_file_name)
    
    return log_file_path

drive = "D:"
folder = "TestLogs"
pn_min = 610721006501
pn_max = 610721006501
year_min = 1998
year_max = 2023
week_min = 1
week_max = 52
test_env_list = ["PTF", "FT", "PI", "XT", "AET"]

if __name__ == "__main__":
  parser = argparse.ArgumentParser()
  parser.add_argument("--count", type=int, default=1, help="The number of times to generate a random folder structure.")
  args = parser.parse_args()

  for i in range(args.count):
    log_file_path = generate_random_folder_structure(drive, folder, pn_min, pn_max, year_min, year_max, week_min, week_max, test_env_list)
    print(log_file_path)