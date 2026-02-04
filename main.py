from io import TextIOWrapper
import tkinter as tk
import threading
import time
import json
from datetime import datetime
import os
import random
from pygame import mixer
import pyautogui
import webbrowser

MENTORSCRIPT_EVERYHOUR_URL = ""
MENTORSCRIPT_EVERY30_URL = ""
SONG_FOLDER = ""
MENTOR_TEXT = ""

HOURLY = 55
THIRTYMINUTE = 30

# Opens the links json and assigns the respective stuff
with open("links.json", "r") as f:
    # For some reason, Windows wants to really be different when opening files... here is a check to ensure cross compat.
    ioText = ""
    if type(f) != TextIOWrapper:
        ioText = f.read()
    else:
        ioText = f
    filein = json.loads(f.read())
    MENTORSCRIPT_EVERYHOUR_URL = filein["MENTORSCRIPT_EVERYHOUR_URL"]
    MENTORSCRIPT_EVERY30_URL = filein["MENTORSCRIPT_EVERY30_URL"]
    SONG_FOLDER = filein["SONG_FOLDER"]
    MENTOR_TEXT = filein["MENTOR_TEXT"]

# Debugging purposes to make sure all songs are gathered properly
print(os.listdir(SONG_FOLDER))

class MentorScriptApp():
    """The Tkinter app that runs the Mentor Script"""
    root: tk.Tk
    backgroundColor: tuple[int, int ,int] = (255,0,0) 
    centerText: tk.Label

    sentOutHourly: bool = False
    sentOutThirty: bool = False

    appThreads: dict[str, threading.Thread] = {}
    currentSong: mixer.Sound
    stillRunning: bool = True


    def sendPrompt(self, toPrompt: str):
        """Opens up a prompt that shows: what the prompt is for, and the ability to stop the song"""

        ## WINDOW
        popup = tk.Toplevel(self.root)
        popup.title("Popup!")
        popup.configure(bg="#92B7D6")
        popup.attributes('-topmost', True)

        ## LABEL
        label = tk.Label(popup, text=toPrompt + "\n(Press OK to close!)", bg="#92B7D6", fg="black", font=("Helvetica", 30), wraplength=600)
        label.pack(pady=20)

        ## BUTTON
        self.currentSong = mixer.Sound("./songs/" + random.choice(os.listdir("./songs")))
        self.currentSong.play()
        def callback():
            popup.destroy()
            mixer.fadeout(400)

        ok_button = tk.Button(popup, highlightbackground="#6499C6",text="OK", command=callback, font=("Helvetica", 25))
        ok_button.pack(pady=10)
        popup.protocol("WM_DELETE_WINDOW", callback)


    def rainbowBackground(self):
        """This runs through as a thread where it changes the background"""
        while True:
            time.sleep(.05)
            if not self.stillRunning:
                return
            now = datetime.now()
            minute = now.minute
            hour = now.hour
                

            r, g, b = self.backgroundColor
            # Simple rainbow shift: cycle through R->G->B
            if r == 255 and g < 255 and b == 0:
                g = min(255, g + 2)
            elif g == 255 and r > 0 and b == 0:
                r = max(0, r - 2)
            elif g == 255 and b < 255 and r == 0:
                b = min(255, b + 2)
            elif b == 255 and g > 0 and r == 0:
                g = max(0, g - 2)
            elif b == 255 and r < 255 and g == 0:
                r = min(255, r + 2)
            elif r == 255 and b > 0 and g == 0:
                b = max(0, b - 2)
            
            if 17 < hour:
                r = 17
                g = 17
                b = 17
                self.centerText.configure(fg="white")
                self.centerText.configure(text="After hours - Mentor Script is inactive")
            else:
                self.centerText.configure(fg="black")
                self.centerText.configure(text=MENTOR_TEXT)

            self.backgroundColor = (r, g, b)
            bg_hex = '#{:02x}{:02x}{:02x}'.format(*self.backgroundColor)
            # Makes the background of both the window and the center text to match
            self.root.configure(bg=bg_hex)
            self.centerText.configure(bg=bg_hex)

    def timeCount(self):
        """Reads the time and see if the minutes match HOURLY or THIRTYMINUTE"""
        while True:
            time.sleep(1)
            if not self.stillRunning:
                return
            now = datetime.now()
            minute = now.minute
            hour = now.hour

            if hour < 10 and 17 < hour:
                return
            if minute == HOURLY and self.sentOutHourly != True:
                print("Sent out the hourly!")
                self.sendPrompt("Hourly headcount!")
                webbrowser.open(MENTORSCRIPT_EVERYHOUR_URL, new=1)
                self.sentOutHourly = True
            if minute == THIRTYMINUTE and self.sentOutThirty != True:
                print("Sent out the thirty!")
                self.sendPrompt("The thirty minute mark! It is TRASH 30!!!")
                webbrowser.open(MENTORSCRIPT_EVERY30_URL, new=1)
                self.sentOutThirty = True
            if not minute == THIRTYMINUTE and not minute == HOURLY:
                self.sentOutHourly = False
                self.sentOutThirty = False

    def backgroundThreads(self):
        """Creates a thread for the processes that need to be threaded, and adds them to appThreads."""
        self.appThreads["rainbowThread"] = threading.Thread(target=self.rainbowBackground, args=())
        self.appThreads["timeCounter"] = threading.Thread(target=self.timeCount, args=())

        # Run threads
        self.appThreads["rainbowThread"].start()
        self.appThreads["timeCounter"].start()

    def shutdown_procedure(self):
        """Shuts down the application by closing off all threads"""
        self.stillRunning = False
        self.root.destroy()

    def keyRelease(self, event):
        """Tests for any key releases"""
        print("LOL!", event.keysym)
        if event.keysym == 'd':
            self.sendPrompt("prompt test")

    def __init__(self):
        """Initalizes the app."""
        self.root = tk.Tk()
        self.root.title("Mentor Script")
        # self.root.state("zoomed")
        self.root.configure(bg="white")
        self.root.geometry(f'{self.root.winfo_screenwidth()}x{self.root.winfo_screenheight()}')
        self.centerText = tk.Label(self.root, text=MENTOR_TEXT, bg="white", fg="black", font=("Helvetica", 32))
        self.backgroundThreads()
        mixer.init()
        self.centerText.place(relx=0.5, rely=0.5, anchor="center")
        self.root.focus_force()
    
    def run(self):
        self.root.protocol("WM_DELETE_WINDOW", self.shutdown_procedure)
        self.root.bind("<KeyRelease>", self.keyRelease)
        self.root.mainloop()

if __name__ == "__main__":
    app = MentorScriptApp()
    app.run()