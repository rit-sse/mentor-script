# Mentor Script
This is the repository for the mentor script!

Simply clone this GitHub repository into a folder on the shared desktop, and run `./run.bat`.

Also, make a shortcut to the `./run.bat` on the desktop for easier access. Please name it something noticeable!
## Setup
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