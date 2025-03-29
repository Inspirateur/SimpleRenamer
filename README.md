# Simple Renamer
A simple and smart bulk file renamer. Just rename 1 file and it will do the rest.

*Blog post: https://teo-orthlieb.github.io/blog/simple-renamer/*

## Preview
![](https://github.com/Inspirateur/SimpleRenamer/blob/master/preview/batch_renamer_demo.gif) 

## Contextual Menu in Windows
1/ Find and open Regedit in start menu  
2/ create the keys if not present: `HKEY_CLASSES_ROOT/*/Shell/Batch Rename/command`  
<sub><sup>(or "Computer/HKEY_CURRENT_USER/Software/Classes\*/shell/Batch rename" if not admin)</sup></sub>  
3/ change default value of `command` to `absolute/path/to/gui.exe "%1"`  
