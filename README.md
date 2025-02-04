# NO LONGER MAINTAINED
This was one of the projects made to learn/get a good feel for rust.
[fd](https://github.com/sharkdp/fd) (even good old regular [gnu
find](https://www.gnu.org/software/findutils/)) lists down directories faster.
Its better to just pipe the unbuffered results of that to sed and fzf and go
from there. Also, i use windows less and less for developing so I don't really
use this anymore.

With that being said, I'm planning to just write scripts for various terminals
that do said fuzzy finding of git repos and cd'ing to it. Name that repo `girep`
or something.

---

# Table of Contents
- [sgr (Search Git Repos)](#sgr-search-git-repos)
- [Dependencies](#dependencies)
    - [Compatible shells](#compatible-shells)
- [Installation](#installation)
    - [Building from source](#building-from-source)
- [Basic Usage](#basic-usage)
- [Optional Commands](#optional-commands)
- [Extras](#extras)
    - [Calling sgr through a shortcut in Windows Terminal](#calling-sgr-through-a-shortcut-in-windows-terminal)
    - [Modify the above two shortcuts to execute something else after **sgr**](#modify-the-above-two-shortcuts-to-execute-something-else-after-sgr)

---
# sgr (Search Git Repos)
**sgr** *(sugar)* is a wrapper around [fzf](https://github.com/junegunn/fzf)
whose search list only includes local git repos in selected directories.
- *selected directories* are defined in `dirs.txt` in the same directory as
**sgr** (auto created)
- *selected directories* can either **contain** git repos **OR** can be git
repos themselves

# Dependencies
1. [fzf](https://github.com/junegunn/fzf)

*You can install fzf with [chocolatey](https://chocolatey.org/)*
```
choco install fzf
```
## Compatible shells
In order to change directories, **sgr** needs a script native to your shell to
be written for it.
1. [cmd](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/cmd)
2. [pwsh](https://github.com/PowerShell/PowerShell) or [powershell](https://github.com/PowerShell/PowerShell)

*You can install pwsh with [chocolatey](https://chocolatey.org/)*
```
choco install pwsh
```

---
# Installation
1. Download the [latest release of **sgr**](https://github.com/saltkid/sgr/releases)
2. Extract the archive
    - You should have these files (windows .7z archive in this example)
    ```
    sgr.bat    <-- the batch script to make sgr work with cmd
    sgr.ps1    <-- the powershell script to make sgr work with pwsh
    sugar.exe  <-- the executable to find git repos
    ```
3. Put these files into one directory you have access to
4. Add that directory to your PATH
    - this is important because the scripts (`.bat` and `.ps1` in this example)
    assume it can execute the `sugar` executable just by calling it.

**Optional**:
- create `dirs.txt` in the same directory as `sgr` and put directories
containing git repos.
    - separate the directories with newlines. example:
    ```
    C:\Users\<username>\spaces are\okay
    C:\Users\<username>\path\to\single\git\repo\is\okay

    %env vars%\are\NOT\okay!! <-- INVALID
    ```
    - if there's no `dirs.txt` in the same directory, `sgr` will create one
    containing only one line:

    if called in `cmd`:
    ```
    %userprofile%\projects
    ```
    if called in `pwsh` or `powershell`:
    ```
    $env:userprofile\projects
    ```
    These environment variables will be expanded to be absolute paths. Below is
    the general format:
    ```
    <whatever is the env var for user>/projects
    ```
    This means **sgr** will only search for git repos in that directory

## Building from Source
If you prefer to build it from source:
1. Clone the repo and cd into it
```
git clone https://github.com/saltkid/sgr.git
cd sgr
```
2. Build it
```
cargo build --release
```
3. Move the `sugar` executable into any directory you have access to
    - You will add this directory to your PATH later
        - or you can put it in a directory already in your PATH
    - the `sugar` executable is in the `./target/release` directory
4. In the `./scripts` directory, pick the scripts that your shell/s use and
move it to the same directory as the `sugar` executable in the previous step
5. If not already, add the directory that contains the `sugar` executable and
the scripts to your PATH

---
# Basic Usage
1. Open a terminal and run `path/to/sgr`
    - your shell will infer that the `sgr` you are referring to is the script
    native to it
        - if you ran `sgr` in `cmd`, it will execute `sgr.bat`
        - if you ran `sgr` in `pwsh`, it will execute `sgr.ps1`
        - etc.
2. This will open up the usual [fzf](https://github.com/junegunn/fzf)
interface. Select a repo and it'll `cd` your current terminal to that path

# Optional Commands
**sgr** has optional commands that handles modifying `dirs.txt`
1. `add`
    - **args**: `path/to/dir`
    - adds `path/to/dir` to `dirs.txt` which will be searched for git repos
    when **sgr** is run

    - **validations**:
        1. you cannot add a path that is already in `dirs.txt`
        2. you cannot add a path that is a subdir of another path already in
        `dirs.txt`
            - this is because **sgr** will search for git repos under
            directories in `dirs.txt` so having a subdirectory of another
            directory will cause double the work for the same result
        3. similar to 2, you cannot add a path that is a parent of another path
        already in `dirs.txt`
2. `remove`
    - **args**: `path/to/dir`, `<int>`, `<int_x>-<int_y>`
    - removes `path/to/dir` from `dirs.txt` if it's in `dirs.txt`
    - if `<int>` is provided, it will remove the dir at line `<int>` in `dirs.txt`
    - if `<int_x>-<int_y>` is provided, it will remove the dirs between lines `<x>` and `<y>` (inclusive)

    - **validations**:
        1. you cannot remove a path that is not in `dirs.txt`
        2. you cannot remove by index where the index is out of range
        3. you cannot remove by range where the range is out of range
        3. you cannot remove by range where the starting range is greater than
        or equal to the ending range (`1-1`, `3-3`, `4-2`)
3. `list`
    - **args**: none, `<int>`, `<int_x>-<int_y>`, `path/pattern`
    - lists `dirs.txt` if no args are given
    - if `<int>` is provided, it will list the dir at line `<int>` in `dirs.txt`
    - if `<int_x>-<int_y>` is provided, it will list the dirs between lines
    `<x>` and `<y>` (inclusive)
    - if `path/pattern` is provided, it will list the dirs that match `path/pattern`
        - case insensitive but must be exact match
    - **validations**:
        1. you cannot list by index where the index is out of range
        2. you cannot list by range where the range is out of range
        3. you cannot list by range where the starting range is greater than or
        equal to the ending range (`1-1`, `3-3`, `4-2`)

### other commands:

4. `help`
    - **args**: none or another command
    - prints help message along with the list of commands
    - if a command is specified, it will print the verbose help message of that
    command
5. `version`
    - **args**: none
    - prints version

---
# Extras
## Calling sgr through a shortcut in Windows Terminal
1. Open Windows Terminal and go to settings (`Ctrl+,`)
2. Open settings.json
3. Under `"actions"`, there are a list of commands. Add either of these two snippets (or both):

Below will just insert the string `sgr` to the terminal and press enter when
you press `ctrl+f`. This will open up the [fzf](https://github.com/junegunn/fzf)
prompt waiting for input.
```
{
    "name": "Find and Goto Git Repository",
    "keys": "ctrl+f"
    "command": { "action": "sendInput", "input": "sgr\u000D" },
},          
```
Below will open a new tab with the [fzf](https://github.com/junegunn/fzf) prompt
waiting for input when you press `ctrl+shift+f`. Difference with above is that
above snippet executes `sgr.bat` in the *same tab* while below snippet executes
on a *new tab*
```
{
    "name": "Find and Goto Git Repository (new tab)",
    "keys": "ctrl+shift+f",
    "command": {
        "action": "newTab",
        "commandline": "cmd.exe /k \"sgr\""
    }
}
```

*Notes:* 
- Just be wary that the first shortcut (sending input) will literally send
"sgr" and press enter, even when you're in another TUI. This is a limitation of
*Windows Terminal* where you can't execute scripts with a shortcut in the same
tab. The next tab variant works as expected though and does not paste text

- If you want to use `pwsh` in the new tab, just replace `cmd.exe /k` with
`C:\\Program Files\\PowerShell\\7\\pwsh.exe -noexit -c`
    - yes, it has to be full path, even if `pwsh` is in your PATH
- if you wish to use `powershell.exe` instead, just replace `cmd.exe /k` with
`powershell.exe -noexit -c`
- `ctrl+shift+f` is by default bound to the action of *opening the search
dialog box* in Windows Terminal. If you want `ctrl+shift+f` for running **sgr**
in *new tab*, remap the `"find"` command to something else like `ctrl+alt+f`
- Windows uses backslashes so those need to be escaped here. Example:
```
"commandline" : "powershell.exe -noexit -c \"sgr\""
```

## Modify the above two shortcuts to execute something else after **sgr**

Instead of just changing directories, in this example, the below snippets will
immediately open up nvim afterwards. Just a small change
```
{
    "name": "Find and Goto Git Repository (nvim)",
    "keys": "ctrl+f",
    "command": { "action": "sendInput", "input": "sgr && nvim .\u000D" },
}, 
```
and 
```
{
    "name": "Find and Goto Git Repository (nvim)",
    "keys": "ctrl+shift+f",
    "command": {
        "action": "newTab",
        "commandline": "cmd.exe /k \"sgr && nvim .\""
    }
}
```
Notice that we just added `&& nvim .` after `sgr` on both shortcuts. That's it.
You can replace `&& nvim .` with any command you want, to execute anything
after **sgr** is done changing directories.

---
# Credits
- [ThePrimeagen](https://github.com/ThePrimeagen) for the idea (specifically, his [tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer), the quick switching part only)
- [fzf](https://github.com/junegunn/fzf)
- [chocolatey](https://chocolatey.org/)
- [cmd](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/cmd)
- [pwsh](https://github.com/PowerShell/PowerShell)
