# WslLink
- A simple Windows batch script for running commands of WSL at Windows side shell (cmd, powershell, etc.)
- Author: snoopy3476@outlook.com


## Description


### What does this script do
- Create and manage symlinks to commands of WSL (inside the directory where the script exists)
- When running WSL commands on Windows shell, help converting Windows path arguments to WSL path if exists
  - Works only if a argument itself is a Windows path (Arguments starting with `[a-zA-Z]:[\/]`, after unquoted)
    - *Ex)*
      - *Input*: `printf "C:\tmp\example-file.txt"`
      - *Output*: `/mnt/c/tmp/example-file.txt`
  - Does not work if a Windows path is embedded inside other strings
    - *Ex)*
      - *Input*: `printf "Test=C:\tmp\example-file.txt"`
      - *Output*: `Test=C:/tmp/example-file.txt`


### Brief usage examples

- Use commands of WSL directly on Windows command prompt
```
C:\>git --version & ls C:\Users
'git' is not recognized as an internal or external command,
operable program or batch file.
'ls' is not recognized as an internal or external command,
operable program or batch file.

C:\>wsllink new git ls
 - Linked command 'git' to WSL.
 - Linked command 'ls' to WSL.

[Command-List]        git       ls

C:\>git --version & ls C:\Users
git version 2.25.1
'All Users'   Default  'Default User'   Public   desktop.ini   snoop

C:\>
```

- Set a WSL program as a default program for some file-extensions
  - When double-click a file on Windows explorer, the file is open with the WSL program directly
  - See [GUI program mode part](#background-process-mode-gui-program-mode) for more information


## Install
- Enable 'Developer Mode' of Windows, before running the script (which is needed for creating symlink)
  - This may be skipped, but then the command management jobs below (creating/deleting commands) should be run as administrator
- Download and move 'wsllink.bat' script to Windows 'PATH' dir
  - It is recommended to create new directory inside the current Windows user home, and add the path to 'PATH' environment variables (ex. `C:\Users\(user-home)\bin` (`%USERPROFILE%\bin`))
    - Make a directory from Windows side shell
      - *Ex)* `mkdir "%USERPROFILE%\bin"`
    - Go to System Properties
      - Press `[Windows key] + [R]` keys
      - Enter `SystemPropertiesAdvanced`
      - Press `[Enter]` key
    - Click 'Environment Variables'
    - Double click 'Path' at user environment variable region, then add a new path inside it
      - *Ex)* `%USERPROFILE%\bin`
    

## Usage
### Basic usage
- Command management
  - Link new commands to WSL:
    - `wsllink new <command-name-1> <command-name-2> ...`
    - `wsllink add <command-name-1> <command-name-2> ...`
    - `wsllink ln <command-name-1> <command-name-2> ...`
  - Unlink existing commands from WSL:
    - `wsllink del <command-name-1> <command-name-2> ...`
    - `wsllink rm <command-name-1> <command-name-2> ...`
  - List linked commands:
    - `wsllink list`
- Command execution
  - `<command-name> <command-arg1> <command-arg2> ...`
  - *Ex)*
    - `emacs ~/src/project1/project1.sh`              *(Using WSL path as arguments)*
    - `emacs C:\Users\(user-home)\bin\wsllink.bat`    *(Using absolute Windows path as arguments)*
    - `emacs bin\wsllink.bat`                         *(Using relative Windows path as arguments)*


### Background process mode (GUI program mode)
- **Note that 'X server for Windows' (VcXsrv, etc.) is required for running GUI programs!**
- Creating and executing a command starts with a dot (.) will make the command (without the leading dot) to be executed as a backgroud process (using tmux)
  - Background process here does not tied to the running shell, so you can close the shell after running it
  - This is useful when you want to execute GUI program of WSL
  - *Ex)*
    - `wsllink new .emacs`        *(Create new link to WSL command 'emacs')*
    - `.emacs bin\wsllink.bat`    *(run 'emacs' at background)*
- Make a shortcut link (.lnk) to the command file `.(command-name).cmd` (**NOT a `.(command-name).bat` file!**) or run it directly, to run GUI programs with mouse click
  - *Ex)*
    - *Add WSL GUI programs to Windows start menu as Windows program*
      1. Go to the folder where the WslLink script exists
      2. Create a shortcut to `.(command-name).cmd` file manually with mouse right click
          - Change the file name which is displayed as program name
          - Set a icon of the link file which is displayed as program icon
          - Set additional binary arguments for the link file if you want
      3. Open the 'Windows start menu' folder
          - Press `[Windows key] + [R]` keys
          - Enter `shell:Start Menu`
          - Press `[Enter]` key
      4. Move the created shortcut in step 2 to the start menu folder
      5. Search and pin the WSL GUI program to Windows start menu
          - Press `[Windows key]` key to open the start menu
          - Enter the name of the link file you made when the start menu is appeared
          - Mouse right click at the searched program, then pin to start menu
    - *Set a WSL GUI program (`.(command-name).cmd`) as a default program ('Open with...') for some file-extensions*
      - After that, the WSL program can open a file directly if you double-click the file at Windows file explorer
        - When a file is executed at Windows file explorer, the path of the file will be passed through cmdline arguments


### Notes for escaping string
- This script converts characters automatically as following:
  - `\` -> `/` (for passing relative path to WSL binaries)
    - To write a literal `\`, you must escape it with another leading `\`: so it will be `\\`
  - `%` -> ` ` (deleted)
    - To write a literal `%`, you must escape it with another leading `%`, so it will be `%%`
  - *Ex)*
    - *Input*: `printf \example\\\\strin\\g:\\ \\'[%%s]\\' "ARG-INPUT"`
    - *(After conversion by the script)*: `printf /example\\strin\g:\ \'[%s]\' "ARG-INPUT"` (<- actual input on WSL shell)
    - *Output*: `/example\string: '[ARG-INPUT]'`
- Of course, special characters of cmd/powershell themselves should be also escaped
