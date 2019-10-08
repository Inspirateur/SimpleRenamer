# SimpleRenamer

<a href="https://www.youtube.com/watch?v=ADsyiEJWdpU&feature=youtu.be">
<img src="https://user-images.githubusercontent.com/27065646/66266362-66a3fa80-e824-11e9-90dd-4fa233ce3652.png" height="40%" width="40%" align="right">
</a>

A prototype for a simple and smart bulk file renamer made with Python 3.7.
The motive behind is that most file renamers are complex tools (often closed source)
that require you to write some kind of regular expression to target the files you want to rename yourself. This is good for professionals, but it's out of reach for a casual user.  
What my software does to address this problem is that **it looks for patterns** in file names itself!

## How to install it:
**Step 1**:  
Get my build for Windows 10 in the releases *(might work with Windows version under 10 I haven't tried)*  
*OR*  
Clone the repo, install the dependencies and build it yourself with `pyinstaller --noconsole main.py`

At this point you can invoke it from the command line and it will execute from the console's location.

**Step 2**:  
If you want the contextual menu accessible with right click, here's how to do it in Windows 10:  
* Search "regedit" on the Windows start menu, run it
* Find `HKEY_CLASSES_ROOT\Directory\Background\shell\`
* Right click on shell -> New -> Key
* Name it however you want, it's the name that will appear in the contextual menu
(I named it "Simple Renamer")
* Right click on the folder you just created -> New -> Key
* Name it "command"
* Click on command, in the right panel there should be a single entry with the values:   
(Default) &emsp; REG_SZ &emsp; (value not set)
* Double click on (Default), this will open a pop-up with 2 fields:  
`Value name` (not editable) and `Value data`
* Put the path to the main.exe you got from Step 1 in value data  
For example mine was (quotes included): `"D:\Repos\SimpleRenamer\dist\main\main.exe"`

And it's done, whenever you right click in a folder of your file explorer like I did in the preview, 
the entry "Simple Renamer" (or whatever name you gave it) should be visible, and clicking on it will execute 
the simple renamer in the folder.


## How to use it:
Right click in a folder with files you want to rename, 
the program will look for patterns in the file titles on its own, 
and display a small window with every pattern it found, as well as a text input field to rename it.
Patterns look like this:
```
Game of Thrones - E/a/S/b/ EnSub 1080p.mkv
```
The `/a/` and `/b/` are **variables**, this could be the episode or season number for example.  
In my example you could rename it to:
```
Game of Thrones - S/b/E/a/.mkv
```
So every title that looked like this:
```
Game of Thrones - E01S8 EnSub 1080p.mkv
```
Will be turned into:
```
Game of Thrones - S8E01.mkv
```

## To-Do:
* Make an installer for Windows, Linux, macOS
* Make the window cleaner/prettier
* Pad the numbers
