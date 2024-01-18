# sift
**sift** is a wrapper around [fzf](https://github.com/junegunn/fzf) whose search list only includes local git repos in selected directories.
- *selected directories* are defined in `dirs.txt` in the same directory as **sift**
- *selected directories* are **not** git repos. They **contain** git repos

## Dependencies
- [fzf](https://github.com/junegunn/fzf) - for the fuzzy findng
- [sed](https://github.com/mbuilov/sed-windows) - for truncating `\.git\` from the found git repos

*You can install both with [chocolatey](https://chocolatey.org/)*
```
choco install fzf sed
```

## Usage
1. Clone the repo **OR** download `sift.bat`
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
    ```

4. Open a terminal and run `path/to/sift.bat`
    - if you have `sift.bat` in your PATH, you can just run `sift`
5. Now, it'll open up the usual [fzf](https://github.com/junegunn/fzf) interface. Select a repo and it'll `cd` your current terminal to that path

## Extra: calling sift through a shortcut in Windows Terminal
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
- both of these snippets assume you have `sift.bat` in your PATH. If not, use `"input": "\"path\\to\\sift.bat\"\u000D"` instead
- `ctrl+shift+f` is by default bound to the action of *opening the search dialog box* in Windows Terminal. If you want `ctrl+shift+f` for running **sift** in *new tab*, remap the `"find"` command to something else like `ctrl+alt+f`
- Windows uses backslashes so those need to be escaped here. Example:
```
"commandline" : "cmd.exe /K \"C:\\Users\\saltkid\\tools\\sift.bat\""
```
The two escaped quotation marks are there incase your path has spaces in it.

## Credits
- [ThePrimeagen](https://github.com/ThePrimeagen) for the idea (specifically, his [tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer))
- [fzf](https://github.com/junegunn/fzf)
- [sed](https://github.com/mbuilov/sed-windows)
- [chocolatey](https://chocolatey.org/)

