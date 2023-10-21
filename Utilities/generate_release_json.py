import json
import argparse
from datetime import datetime

"""

{
  "version": "9.9.9",
  "notes": "See the assets to download this version and install.",
  "pub_date": "2023-06-13T05:12:10.282Z",
  "platforms": {
    "linux-x86_64": {
      "signature": "",
      "url": "https://github.com/HuakunShen/<repo>/releases/download/<version>/<file>_amd64.AppImage.tar.gz"
    },
    "windows-x86_64": {
      "signature": "==",
      "url": "https://github.com/HuakunShen/<repo>/releases/download/<version>/<file>_x64_en-US.msi.zip"
    },
    "darwin-aarch64": {
      "signature": "",
      "url": "https://github.com/HuakunShen/<repo>/releases/download/<version>/<file>_universal.app.tar.gz"
    },
    "darwin-x86_64": {
      "signature": "",
      "url": "https://github.com/HuakunShen/<repo>/releases/download/<version>/<file>_universal.app.tar.gz"
    }
  }
}


"""


def generate_json(version, pub_date, platforms):
    """_summary_

    Args:
        version (_type_): _description_
        pub_date (_type_): _description_
        platforms (_type_): _description_

    Returns:
        _type_: _description_
    """
    data = {
        "version": version,
        "notes": "See the assets to download this version and install.",
        "pub_date": pub_date,
        "platforms": {}
    }
    for platform, url in platforms:
        data["platforms"][platform] = {
            "signature": "PLACEHOLDER_SIGNATURE_FOR_{}".format(platform),
            "url": url
        }
    return json.dumps(data, indent=2)

def main():
    parser = argparse.ArgumentParser(description='Generate specified .json format.')
    parser.add_argument('--version', required=True, help='Version of the release')
    parser.add_argument('--pub-date', default=datetime.utcnow().isoformat() + 'Z', help='Publication date in ISO format')
    parser.add_argument('--platform', action='append', nargs=2, metavar=('PLATFORM', 'URL'), help='Platform and associated URL', required=True)
    parser.add_argument('--output', default="releases_tauri.json", help='Output file name (default: output.json)')

    args = parser.parse_args()

    json_output = generate_json(args.version, args.pub_date, args.platform)

    with open(args.output, 'w') as file:
        file.write(json_output)

    print(f"Data written to {args.output}")

if __name__ == "__main__":
    main()
