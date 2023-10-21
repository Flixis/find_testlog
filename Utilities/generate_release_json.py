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
def generate_json(version, pub_date, platform_data):
    data = {
        "version": version,
        "notes": "See the assets to download this version and install.",
        "pub_date": pub_date,
        "platforms": {
            "linux-x86_64": {"signature": "PLACEHOLDER_SIGNATURE", "url": "PLACEHOLDER_URL"},
            "windows-x86_64": {"signature": "PLACEHOLDER_SIGNATURE", "url": "PLACEHOLDER_URL"},
            "darwin-aarch64": {"signature": "PLACEHOLDER_SIGNATURE", "url": "PLACEHOLDER_URL"},
            "darwin-x86_64": {"signature": "PLACEHOLDER_SIGNATURE", "url": "PLACEHOLDER_URL"}
        }
    }

    for platform, url, signature in platform_data:
        if platform in data["platforms"]:
            data["platforms"][platform]["url"] = url
            data["platforms"][platform]["signature"] = signature

    return json.dumps(data, indent=2)

def main():
    parser = argparse.ArgumentParser(description='Generate specified .json format.')
    parser.add_argument('--version', required=True, help='Version of the release')
    parser.add_argument('--pub-date', default=datetime.utcnow().isoformat() + 'Z', help='Publication date in ISO format')
    parser.add_argument('--platform', action='append', nargs=3, metavar=('PLATFORM', 'URL', 'SIGNATURE'), help='Platform, associated URL, and signature', required=True)
    parser.add_argument('--output', default="releases_tauri.json", help='Output file name (default: output.json)')

    args = parser.parse_args()

    json_output = generate_json(args.version, args.pub_date, args.platform)

    with open(args.output, 'w') as file:
        file.write(json_output)

    print(f"Data written to {args.output}")

if __name__ == "__main__":
    main()