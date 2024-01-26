# sift
**sift** is a wrapper around [fzf](https://github.com/junegunn/fzf) whose search list only includes local git repos in selected directories.
- *selected directories* are defined in `dirs.txt` in the same directory as **sift**
- *selected directories* **contain** git repos **AND** can be git repos themselves

## Dependencies (must be installed and added to your PATH)
- [fzf](https://github.com/junegunn/fzf)
- [sed](https://github.com/mbuilov/sed-windows)

*You can install both with [chocolatey](https://chocolatey.org/)*
```
choco install fzf sed
```

## Getting Started
### Installation
1. Download `sift.bat`
2. ***Optionally*** add `sift.bat` to your PATH
3. ***Optionally*** create `dirs.txt` in the same directory as `sift.bat` and put directories containing git repos.
    - if there's no `dirs.txt` in the same directory as `sift.bat`, it will create one containing only one line:
      ```
      %userprofile%\projects
      ```
      This means **sift** will only search for git repos in that directory
    - separate the directories with newlines
    - remember to use backslashes and avoid special characters in the paths
    - example:
    ```
    %userprofile%\projects
    C:\Users\<username>\spaces and abs paths\are\okay
    %other_env_vars%\are\also\okay
    %userprofile%\path\to\single\git\repo\is\okay
    ```
### Basic Usage
1. Open a terminal and run `path/to/sift.bat`
    - if you have `sift.bat` in your PATH, you can just run `sift`
2. Now, it'll open up the usual [fzf](https://github.com/junegunn/fzf) interface. Select a repo and it'll `cd` your current terminal to that path

### Commands
**sift** has optional commands that handles modifying `dirs.txt`
1. `add`
    - **args**: `path/to/dir`
    - adds `path/to/dir` to `dirs.txt` which will be searched for git repos when **sift** is run
2. `remove`
    - **args**: `path/to/dir`, `<int>`, `<int_x>-<int_y>`
    - removes `path/to/dir` from `dirs.txt` if it's in `dirs.txt`
    - if `<int>` is provided, it will remove the dir at line `<int>` in `dirs.txt`
    - if `<int_x>-<int_y>` is provided, it will remove the dirs between lines `<x>` and `<y>`
3. `list`
    - **args**: none, `<int>`, `<int_x>-<int_y>`
    - lists `dirs.txt` if no args are given
    - if `<int>` is provided, it will list the dir at line `<int>` in `dirs.txt`
    - if `<int_x>-<int_y>` is provided, it will list the dirs between lines `<x>` and `<y>`

## Extra
### 1. Calling sift through a shortcut in Windows Terminal
1. Add **sift** to your PATH **OR** just keep in mind its location
2. Open Windows Terminal and go to settings (`Ctrl+,`)
3. Open settings.json
4. Under `"actions"`, there are a list of commands. Add either of these two snippets (or both):

Below will send the string `sift` to the terminal and press enter when you press `ctrl+f`. This will open up the [fzf](https://github.com/junegunn/fzf) prompt waiting for input.
```
{
    "name": "Find and Goto Git Repository",
    "keys": "ctrl+f"
    "command": { "action": "sendInput", "input": "sift\u000D" },
},          
```
Below will open a new tab with the [fzf](https://github.com/junegunn/fzf) prompt waiting for input when you press `ctrl+shift+f`. Difference with above is that above snippet executes `sift.bat` in the *same tab* while below snippet executes on a *new tab*
```
{
    "name": "Find and Goto Git Repository (new tab)",
    "keys": "ctrl+shift+f",
    "command": {
        "action": "newTab",
        "commandline": "cmd.exe /K sift"
    }
}
```

*Notes:* 
- these extras assume you have `sift.bat` in your PATH. If not, use `"input": "\"path\\to\\sift.bat\"\u000D"` instead
- `ctrl+shift+f` is by default bound to the action of *opening the search dialog box* in Windows Terminal. If you want `ctrl+shift+f` for running **sift** in *new tab*, remap the `"find"` command to something else like `ctrl+alt+f`
- Windows uses backslashes so those need to be escaped here. Example:
```
"commandline" : "cmd.exe /K \"C:\\Users\\username\\tools\\sift.bat\""
```
The two escaped quotation marks are there incase your path has spaces in it.

### 2. Modify the above two shortcuts to execute something else after **sift**

Instead of just changing directories, in this example, the below snippets will immediately open up nvim afterwards. Just a small change
```
{
    "name": "Find and Goto Git Repository (nvim)",
    "keys": "ctrl+f",
    "command": { "action": "sendInput", "input": "sift && nvim .\u000D" },
}, 
```
and 
```
{
    "name": "Find and Goto Git Repository (nvim)",
    "keys": "ctrl+shift+f",
    "command": {
        "action": "newTab",
        "commandline": "cmd.exe /K sift && nvim ."
    }
}
```
Notice that we just added `&& nvim .` after `sift` on both shortcuts. That's it. You can replace `&& nvim .` with any command you want, to execute anything after **sift** is done changing directories.

## Credits
- [ThePrimeagen](https://github.com/ThePrimeagen) for the idea (specifically, his [tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer))
- [fzf](https://github.com/junegunn/fzf)
- [sed](https://github.com/mbuilov/sed-windows)
- [chocolatey](https://chocolatey.org/)

