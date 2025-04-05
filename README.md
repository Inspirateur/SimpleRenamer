# Simple Renamer
A simple and smart bulk file renamer. Just rename 1 file and it will do the rest.

*Blog post: https://teo-orthlieb.github.io/blog/simple-renamer/*

## Preview
![](https://github.com/Inspirateur/SimpleRenamer/blob/master/preview/batch_renamer_demo.gif) 

## Install on Windows
Install [Scoop](https://scoop.sh/) if you don't already have it - In a Powershell:
```ps1
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
```

You can then install SimpleRenamer with:  
```
scoop install -s https://raw.githubusercontent.com/Inspirateur/SimpleRenamer/refs/heads/master/SimpleRenamer.json
```

This will create a "Batch Rename" entry in the context menu, if you don't want it find and open "Regedit" and delete the `HKCU:\Software\Classes\*\Shell\Batch Rename` key.

You can uninstall it with `scoop uninstall SimpleRenamer`

## Install on other platforms
I don't have any prebuilt binaries for other platform, so you will have to:
1. install rust with https://rustup.rs/
2. clone the repository to have the source code on your machine
3. in the repository folder run `cargo build --release`
   
This should compile SimpleRenamer and produce a "gui.exe" binary you can use on your machine. You can add it to the Path or create an alias if you want to invoke it from the command line.
