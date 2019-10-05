# SimpleRenamer
A prototype for a simple and smart bulk file renamer made with Python 3.7

Presentation: https://www.youtube.com/watch?v=ADsyiEJWdpU&feature=youtu.be

## How to install it:
**Step 1**:

You can either clone the repos, install the dependencies and build it yourself with pyinstaller main.py
OR 
Get my build for Windows 10 in the releases *(might work with Windows version under 10 I haven't tried)*

At this point you can invoke it from the command line and it will execute from the console's location.

**Step 2**:

If you want the contextual menu accessible with right click, here's how to do it in Windows 10:

* Search "regedit" on the Windows start menu, run it
* Find HKEY_CLASSES_ROOT\Directory\Background\shell\
* Right click on shell -> New -> Key
* Name it however you want, it's the name that will appear in the contextual menu
(I named it "Simple Renamer")
* Right click on the folder you just created -> New -> Key
* Name it "command"
* Click on command, in the right panel there should be a single entry with the values: 
(Default)    REG_SZ    (value not set)
* Double click on (Default), this will open a pop-up with 2 fields:
Value name (not editable) and  Value data
* Put the path to the main.exe you got from Step 1 in value data
For example mine was (quotes included): "D:\Repos\SimpleRenamer\dist\main\main.exe"

And it's done, whenever you right click in a folder of your file explorer like I did in the preview, 
the entry "Simple Renamer" (or whatever name you gave it) should be visible, and clicking on it will execute 
the simple renamer in the folder.
