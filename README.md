# sift
**sift** is a wrapper around fzf whose search list only includes local git repos in selected directories.
    - selected directories are defined in `dirs.txt` in the same directory as **sift**
    - by default, the only directory there is `C:\Users\<username>\projects` where `<username>` is the name of the current user

## Dependencies
- [fzf](https://github.com/junegunn/fzf)- for the fuzzy findng
- [sed](https://github.com/mbuilov/sed-windows) - for truncating `\.git\` from the found git repos
*You can install both with [chocolatey](https://chocolatey.org/)*
```
choco install fzf sed
```

## Usage
1. Clone the repo or download: `sift.bat` and `dirs.txt` and put them in the same directory
2. ***Optionally*** add `sift.bat` to your PATH
3. ***Optionally*** add more directories you would like to search in in `dirs.txt`
    - by default, it only contains `C:\Users\<username>\projects` so **sift** will only search there
    - separate the directories with newlines
    - remember to use backslashes and avoid special characters in the paths
    - example:
    ```
    %userprofile%\projects
    C:\Users\<username>\spaces and abs paths\are\okay
    %other_env_vars%\are\also\okay
    ```
4. Open a terminal and run `path/to/sift.bat`
    - if you have `sift.bat` in your PATH, you can just run `sift`
5. Now, it'll open up the usual fzf interface. Select a repo and it'll `cd` your current terminal to that path

## Extra: calling sift through a shortcut in Windows Terminal
*Note that this will open the selected repo in a new tab. This is a limitation of Windows Terminal*
1. Ensure you have **sift** added to your PATH **OR** just keep in mind where you put it for later
2. Open Windows Terminal and go to settings
3. Open settings.json
4. Under `"actions"`, there are a list of commands. Add this snippet to the list:
```json
{
    "name": "Find and Goto Git Repository",
    "keys": "ctrl+f",
    "command": {
        "action": "newTab",
        "commandline": "cmd.exe /K \"path\\to\\sift.bat\""
    }
},
```
Now, when you press `ctrl+f`, this will open a **new tab** with the [fzf](https://github.com/junegunn/fzf) prompt waiting for input. Unfortunately (or fortunately depending on you), you cannot map a shortcut to execute anything on the same tab. You can do it in a new tab, new pane, etc.

Reminder that Windows uses backslashes so those need to be escaped. Example:

```
"commandline" : "cmd.exe /K \"C:\\Users\\saltkid\\tools\\sift.bat\""
```
The two escaped quotation marks are there incase your path has spaces in it.

## Credits
- [ThePrimeagen](https://github.com/ThePrimeagen) for the idea (specifically, his [tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer)
- [fzf](https://github.com/junegunn/fzf)
- [sed](https://github.com/mbuilov/sed-windows)
- [chocolatey](https://chocolatey.org/)

