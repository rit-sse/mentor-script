from io import TextIOWrapper
import tkinter as tk
import threading
import time
import json
from datetime import datetime
import os
import random
import pygame
import pyautogui
import webbrowser
import csv

MENTORSCRIPT_EVERYHOUR_URL = ""
MENTORSCRIPT_EVERY30_URL = ""
SONG_FOLDER = ""
MENTOR_TEXT = ""

HOURLY = 55
THIRTYMINUTE = 30

#CSV Format: id,name,email,exam checked out,class_number,date checked out
CSV_READER = ""
CSV_WRITER = ""
CSV_APPENDER = ""

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

# Assigns access to read the csv
with open("database.csv", "r", newline="") as csvfile:
    CSV_READER = csv.DictReader(csvfile)

    # Test: Print each row in the CSV file (works!)
    for row in CSV_READER:
        print(row["name"], row["id"]) # Each row is a list of strings
        print(row)

# Assigns access to edit the csv
with open("database.csv", "w", newline="") as csvfile:
    fieldnames = ["id", "name", "email", "exam_checked_out", "class_number", "date_checked_out"]
    CSV_WRITER = csv.DictWriter(csvfile, fieldnames=fieldnames)

    # Test: Print each row in the CSV file (don't work bc I'm using with open() wrong)
    CSV_WRITER.writeheader()
    CSV_WRITER.writerow({"id": "www2222", "name": "potatolover", "email": "potatowww2222@rit.edu"})
    for row in CSV_READER:
        if row["id"] == "www2222":
            CSV_WRITER.writerow({"exam_checked_out": "how to eat water", "class_number": "WAT-1212", "date_checked_out": "09/07/2025"})


# Assigns access to add data to the csv
with open("database.csv", "a", newline="") as csvfile:
    CSV_APPENDER = csv.writer(csvfile)

    # Test: Write some stuff into the csv (works!)
    CSV_APPENDER.writerow(["aha1234", "Addison Asu", "randomemail4321@rit.edu"])
    CSV_APPENDER.writerow(["rrr4321", "Ryan Reynolds", "randomemail4321@rit.edu", "How to eat dirt the right way", "POTATO-420","09/07/2025"])

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
    currentSong: pygame.mixer.Sound
    stillRunning: bool = True

    def sendPrompt(self, toPrompt: str):
        """Opens up a prompt that shows: what the prompt is for, and the ability to stop the song"""

        ## WINDOW
        popup = tk.Toplevel(self.root)
        popup.title("Popup!")
        popup.configure(bg="#92B7D6")

        ## LABEL
        label = tk.Label(popup, text=toPrompt, bg="#92B7D6", fg="black", font=("Helvetica", 30), wraplength=600)
        label.pack(pady=20)

        ## BUTTON
        self.currentSong = pygame.mixer.Sound("./songs/" + random.choice(os.listdir("./songs")))
        self.currentSong.play()
        def callback():
            popup.destroy()
            pygame.mixer.fadeout(400)

        ok_button = tk.Button(popup, highlightbackground="#6499C6",text="OK", command=callback, font=("Helvetica", 25))
        ok_button.pack(pady=10)
        popup.protocol("WM_DELETE_WINDOW", callback)
    

    def readID(self, input_id):
        """This takes in an id and opens up different prompts depending on the status of the id"""

        # Functions to access CSV
        def search_csv(id):
            """search for a person in the csv, and if one is found return their information"""
            for row in CSV_READER:
                if row["id"] == id:
                    return row

        def add_to_csv(id, name, email):
            """Adds a new person to the csv file"""
            CSV_APPENDER.writerow([id, name, email])
        
        def edit_csv(id="", name="", email="", exam="", class_number="", date=""):
            """edit a person's data in the csv"""
            for row in CSV_READER:
                if row["id"] == id:
                    CSV_WRITER.writerow({"id": id, "name": name, "email": email, "exam_checked_out": exam, "class_number": class_number, "date_checked_out": date})
    
        # check if ID is in the csv
        if search_csv(input_id) != "": # id is in the csv
            if search_csv(input_id)[3] != "": # the exam is the 3rd index in the list of strings, if that index is returned "" there is no exam checked out
                # if exam already checked out:
                    # Popup:
                        # (name of checked out exam) output
                        # [check in exam?] prompt input
                        # SUBMIT button
                checked_out_exam = search_csv(id)[3]


                ## WINDOW
                check_in_popup = tk.Toplevel(self.root)
                check_in_popup.title("Check_In_Popup!")
                check_in_popup.configure(bg="#92B7D6")

                ## LABEL
                exam_label = tk.Label(check_in_popup, text="Checked-Out Exam: " + checked_out_exam, bg="#92B7D6", fg="black", font=("Helvetica", 30), wraplength=600)
                exam_label.pack(pady=20)

                ## RADIOBUTTON
                check_in_radiobutton = tk.Radiobutton(check_in_popup, highlightbackground="#6499C6", text="Check in exam?", font=("Helvetica", 25))
                check_in_radiobutton.pack(pady=10)

                ## BUTTON
                def callback():
                    check_in_popup.destroy()
                
                def submit():
                    edit_csv(id=input_id, exam="", class_number="", date="") # removes exam, class_number, and date for the specified person

                submit_button = tk.Button(check_in_popup, highlightbackground="#6499C6", text="SUBMIT", command=submit, font=("Helvetica", 25))
                submit_button.pack(pady=10)

                check_in_popup.protocol("WM_DELETE_WINDOW", callback)
                
            elif search_csv(input_id)[3] == "":
                # if exam not checked out, if/when the ID is in the database:
                    # Popup:
                        # [name of exam that is being checked out] input
                        # [class number of the exam (ex: SWEN-124)] input
                        # SUBMIT button
                pass

        else: # id is not in the csv
            # if ID not in database(csv file?), then open popup to create new section in database(new line in csv file)
                # Database holds data of who has what exam checked out
                # Popup has two inputs:
                    # Student name (First and Last)
                    # Student rit email
                    # SUBMIT button
            pass
            

        

    def rainbowBackground(self):
        """This runs through as a thread where it changes the background"""
        while True:
            time.sleep(.05)
            if not self.stillRunning:
                break
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
                break
            now = datetime.now()
            minute = now.minute
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
    
    def mouseMoveIdle(self):
        while True:
            for i in range(1, 10):
                time.sleep(0.2)
                pyautogui.moveRel(0, -1)
                time.sleep(0.2)
                pyautogui.moveRel(1, 0)
                time.sleep(0.2)
                pyautogui.moveRel(0, 1)
                time.sleep(0.2)
                pyautogui.moveRel(-1, 0)
            time.sleep(60 * 5)

    def backgroundThreads(self):
        """Creates a thread for the processes that need to be threaded, and adds them to appThreads."""
        self.appThreads["rainbowThread"] = threading.Thread(target=self.rainbowBackground, args=())
        self.appThreads["timeCounter"] = threading.Thread(target=self.timeCount, args=())
        self.appThreads["idleMover"] = threading.Thread(target=self.mouseMoveIdle, args=())

        # Run threads
        self.appThreads["rainbowThread"].start()
        self.appThreads["timeCounter"].start()
        self.appThreads["idleMover"].start()

    def shutdown_procedure(self):
        """Shuts down the application by closing off all threads"""
        self.stillRunning = False

    def keyRelease(self, event):
        """Tests for any key releases"""
        print("LOL!", event.keysym)
        if event.keysym == 'd':
            self.sendPrompt("prompt test")

    def __init__(self):
        """Initalizes the app."""
        self.root = tk.Tk()
        self.root.title("Mentor Script")
        self.root.state("zoomed")
        self.root.configure(bg="white")
        self.centerText = tk.Label(self.root, text=MENTOR_TEXT, bg="white", fg="black", font=("Helvetica", 32))
        self.backgroundThreads()
        pygame.mixer.init()
        self.centerText.place(relx=0.5, rely=0.5, anchor="center")
    
    def run(self):
        self.root.protocol("WM_DELETE_WINDOW", self.shutdown_procedure)
        self.root.bind("<KeyRelease>", self.keyRelease)
        self.root.mainloop()

if __name__ == "__main__":
    app = MentorScriptApp()
    app.run()