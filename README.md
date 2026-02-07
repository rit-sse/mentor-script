# Mentor Script
This is the repository for the mentor script!

## Installation

### Quick Install (Recommended)
Run this one-liner to download and install the latest release:
```bash
curl -fsSL https://raw.githubusercontent.com/rit-sse/mentor-script/main/install.sh | bash
```

This will install `mentor-script` to `/usr/local/bin` so you can run it from anywhere.

### Manual Installation
1. Download the latest release binary for your platform from the [releases page](https://github.com/rit-sse/mentor-script/releases)
2. Move it to `/usr/local/bin/mentor-script`
3. Make it executable: `chmod +x /usr/local/bin/mentor-script`

### Development Setup
If you want to run from source:
```bash
git clone https://github.com/rit-sse/mentor-script.git
cd mentor-script
./run.sh
```

## Setup
### Rust
Create "config.json" in the mentor script directory (/usr/local/bin/). It should use this format:
```json
{
  "mentor_text": "Hello Mentor!",
  "hourly_link": "HOURLY_GOOGLE_FORMS",
  "thirty_link": "30_MIN_GOOGLE_FORMS"
}
```
### Python
Create "links.json" in the mentor script directory. Here is layout that it should follow:
```json
{
    "MENTORSCRIPT_EVERYHOUR_URL": "HOURLY_GOOGLE_FORMS",
    "MENTORSCRIPT_EVERY30_URL": "30_MIN_GOOGLE_FORMS",
    "SONG_FOLDER": "./songs",
    "MENTOR_TEXT": "Hello Mentor!"
}
```
## Contributing
**NOTE**: If you are doing adjustments on the mentor script, it is **HIGHLY** recommended to create a branch to do updates before bringing it to the main branch. The mentor script runs the auto-updater first to check for any changes on the main branch before launching the mentor script.


When you are done with adjusting the mentor script, create a pull request and ask the Tech Head to review!