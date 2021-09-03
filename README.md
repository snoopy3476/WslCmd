# WslLink
- A simple Windows batch script for running commands of WSL at Windows side shell (cmd, powershell, etc.)
- Author: snoopy3476@outlook.com


## What does this script do
- Create and manage symlinks to commands of WSL (inside the directory where the script exists)
- (When running WSL commands on Windows shell) Help converting Windows path to WSL path, if exists in some of arguments
  - Works only if a argument itself is a Windows path (Starting with `[a-zA-Z]:[\/]`)
    - Ex)
      - Input: `printf "C:\tmp\example-file.txt"`
      - Output: `/mnt/c/tmp/example-file.txt`
  - Does not work if a Windows path is embedded inside other strings
    - Ex)
      - Input: `printf "Test=C:\tmp\example-file.txt"`
      - Output: `Test=C:/tmp/example-file.txt`


## Install
- Enable 'Developer Mode' of Windows, before running the script (which is needed for creating symlink)
  - You can skip this, but you have to do command management below (creating/deleting commands) as administrator if developer mode is not enabled
- Download and move 'wsllink.bat' script to 'PATH'
  - It is recommended to create new directory inside the current Windows user home, and add the path to 'PATH' environment variables (ex. `C:\Users\(user-home)\bin`)
    - Make directory
      - Ex) `mkdir C:\Users\(user-home)\bin`
    - Go to System Properties
      - Press `[Windows key] + [R]` keys
      - Enter `SystemPropertiesAdvanced`
      - Press `[Enter]` key
    - Click 'Environment Variables'
    - Double click 'Path' at user environment variable region, then add a new path inside it
      - Ex) `C:\Users\(user-home)\bin`
    

## Usage
### Basic usage
- Command management
  - Install new command (of WSL):
    - `wsllink new <command-name>`
    - `wsllink add <command-name>`
    - `wsllink ln <command-name>`
  - Uninstall existing command:
    - `wsllink del <command-name>`
    - `wsllink rm <command-name>`
  - List installed commands:
    - `wsllink list`
- Command execution
  - `<command-name> <command-arg1> <command-arg2> ...`
  - Ex)
    - `emacs ~/src/project1/project1.sh`             :: (Using WSL path as arguments)
    - `emacs C:\Users\(user-home)\bin\wsllink.bat`   :: (Using absolute Windows path as arguments)
    - `emacs bin\wsllink.bat`                        :: (Using relative Windows path as arguments)


### Background process mode (GUI program mode)
- If you create and execute a command starts with a dot (.), then the command (without the leading dot) is executed as a backgroud process (using tmux).
  - Background process here does not tied to the running shell, so you can close the shell after running it
  - This is useful when you want to execute GUI program of WSL through 'X server for Windows' (vcxsrv, etc.)
  - Ex)
    - `wsllink new .emacs`        :: (Create new link to WSL command 'emacs')
    - `.emacs bin\wsllink.bat`    :: (run 'emacs' at background)
- You can make a shortcut link (.lnk) to the command file `.(command-name).cmd` (NOT a `.(command-name).bat` file!) or run it directly, to run GUI programs with mouse click
  - Ex)
    - Set WSL GUI program in Windows start menu as Windows program
      1. Go to the folder where the WslLink script exists
      2. Create a shortcut to `.(command-name).cmd` file manually with mouse right click
          - Change the file name as you want
          - Set a icon of the link file if you want
          - Set additional binary arguments for the link file if you want
      3. Open the 'Windows start menu' folder
          - Press `[Windows key] + [R]` keys
          - Enter `shell:Start Menu`
          - Press `[Enter]` key
      4. Move the created shortcut in step 2 to the start menu folder
      5. You can now Search and place the WSL GUI program at Windows start menu
          - Press `[Windows key]` key to open the start menu
          - Enter the name of the link file you made when the start menu is appeared
          - Mouse right click at the searched program, then pin to start menu
    - Set the default program (Open with...) to `.(command-name).cmd` for some file-extensions
      - After that, a WSL program can open a file directly if you double click the file at Windows file explorer


### Notes for escaping string
- This script converts characters automatically as following:
  - `\` -> `/` (for passing relative path to WSL binaries)
    - To write a literal `\`, you must escape it with another leading `\`: so it will be `\\`
  - `%` -> ` ` (deleted)
    - To write a literal `%`, you must escape it with another leading `%`, so it will be `%%`
  - Ex)
    - Input: `printf \example\\\\strin\\g:\\ \\'[%%s]\\' "ARG-INPUT"`
    - (After conversion by the script) : `printf /example\\strin\g:\ \'[%s]\' "ARG-INPUT"` (<- actual input on WSL shell)
    - Output: `/example\string: '[ARG-INPUT]'`
- Of course, special characters of cmd/powershell themselves should be also escaped
