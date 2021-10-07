# WslLink
- A simple executable binary for running commands/programs of WSL at Windows side (cmd, powershell, icon-click, etc.)
- Author: snoopy3476@outlook.com


## Description


### What does this executable do
- Create and manage symlinks to commands of WSL (inside the directory where the executable binary exists)
- When running WSL commands on Windows shell, help converting Windows path arguments to WSL path if exists
  - This auto-conversion Works only if an argument itself is a Windows path (Arguments starting with `[a-zA-Z]:[\/]`, after unquoted)
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
C:\>wsllink list
 - WSL command list (wsllink) :  
 
C:\>git --version & ls C:\Users
'git' is not recognized as an internal or external command,
operable program or batch file.
'ls' is not recognized as an internal or external command,
operable program or batch file.

C:\>wsllink new git ls
 - Linked command(s) successfully

C:\>wsllink list
 - WSL command list (wsllink) :  'git'  'ls'

C:\>git --version & ls C:\Users
git version 2.25.1
'All Users'   Default  'Default User'   Public   desktop.ini   snoop

C:\>
```

- Set a WSL program as a default program for some file-extensions
  - When double-click a file on Windows explorer, the file is open with the WSL program directly
  - See [Detached process mode (GUI program mode) part](#detached-process-mode-gui-program-mode) for more information


## Prerequisites
- Enable 'Developer Mode' of Windows, before using WslLink (which is needed for creating symlink in Windows)
  - This may be skipped, but then the command management jobs below (creating commands) should be run as administrator


## Build & Install
- Download a pre-compiled executable binary from [Releases](https://github.com/snoopy3476/WslLink/releases).
  - Make folder for WslLink
  - Put the downloaded binary into the created folder
  - Append the folder path to Windows PATH env var
    ```
    set WSLLINK_ROOT="C:\Users\(USER_NAME)\WslLink"
    setx PATH "%PATH%;%WSLLINK_ROOT%"
    ```
- Build on Windows (may need to install Microsoft Visual Studio)
  - [Install Rust for Windows](https://www.rust-lang.org/tools/install)
  - Build and install with Cargo on Windows CMD
    ```
    :: Run following commands on Windows CMD
    
    :: Set a path of Windows to install: Replace (USER_NAME) appropriately
    set WSLLINK_ROOT="C:\Users\(USER_NAME)\WslLink"

    :: Install from the repository, then append installed path to Windows PATH env var
    cargo install --git=https://github.com/snoopy3476/WslLink.git ^
                  --root="%WSLLINK_ROOT%" ^
    && setx PATH "%PATH%;%WSLLINK_ROOT%\bin"
    ```
  - Re-open CMD to apply modified PATH
    - If you are using Windows Terminal, it may not be enough to open a new CMD tab.
      You need to close and re-open Windows Terminal itself.
- Build on WSL (need to install mingw-64)
  - [Install Rust for WSL](https://www.rust-lang.org/tools/install)
  - Install prerequisites on WSL
    - Install mingw-64
      ```
      # Example on WSL Debian / Ubuntu
      sudo apt install mingw-w64
      ```
    - Install rustup target for cross-compile
      ```
      rustup target add x86_64-pc-windows-gnu
      ```
  - Build on WSL & install to Windows with Cargo
    ```
    # Set a path of Windows to install: Replace (USER_NAME) appropriately
    WSLLINK_ROOT="C:\\Users\\(USER_NAME)\\WslLink"

    # Install from the repository, then append installed path to Windows PATH env var
    cargo install --git=https://github.com/snoopy3476/WslLink.git \
                  --target=x86_64-pc-windows-gnu \
                  --root="$(wslpath $WSLLINK_ROOT)" \
    && printf 'setx PATH "%%PATH%%;%s\\bin"\n' "$WSLLINK_ROOT" | cmd.exe; echo
    ```
  - Re-open CMD to apply modified PATH
    - If you are using Windows Terminal, it may not be enough to open a new CMD tab.
      You need to close and re-open Windows Terminal itself.


## Uninstall
1. Remove WslLink root folder
2. Remove WslLink bin folder from Windows PATH env vars
   1. Go to System Properties
      1. Press `[Windows key] + [R]` keys
      2. Enter `SystemPropertiesAdvanced`
      3. Press `[Enter]` key
   2. Click 'Environment Variables'
   3. Double click 'Path' at user environment variable region, find the WslLink path, and remove it
      - *Ex)* `C:\Users\(USER_NAME)\WslLink\bin`


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
    - `wsllink li`
    - `wsllink list`
- Command execution
  - `<command-name> <command-arg1> <command-arg2> ...`
  - *Ex)*
    - `emacs ~/src/project1/project1.sh`           *(Using WSL path as arguments)*
    - `emacs C:\Users\(user-home)\bin\test.txt`    *(Using absolute Windows path as arguments)*
    - `emacs bin\test.txt`                         *(Using relative Windows path as arguments)*


### Detached process mode (GUI program mode)
- **Note that 'X server for Windows' (WSLg, VcXsrv, etc.) is required for running GUI programs!**
- Running a command starting with a '!' will run the command as a detached, backgroud process
  - Detached background process here does not tied to the running shell, so you can close the shell after running it
  - This is useful when you want to execute GUI program of WSL
  - *Ex)*
    - `wsllink new emacs`        *(Create new links to WSL command 'emacs')*
    - `!emacs bin\test.txt`      *(run 'emacs' at background)*
- Make a shortcut link (.lnk) to the command file `(command-name).exe` (**NOT a `!(command-name).exe` file!**) or run it directly, to run GUI programs with mouse click
  - *Ex)*
    - *Add WSL GUI programs to Windows start menu as Windows program*
      1. Go to the folder where the WslLink script exists
      2. Create a shortcut to `(command-name).exe` file manually with mouse right click
          - Change the shortcut file name which is displayed as program name
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
    - *Set a WSL GUI program symlink (`(command-name).exe`) as a default program ('Open with...') for some file-extensions*
      - After that, the WSL program can open a file directly if you double-click the file at Windows file explorer
        - When a file is executed at Windows file explorer, the path of the file will be passed through cmdline arguments

<!--
### Notes for escaping string
- This script converts characters automatically as following:
  - `\` -> `/` (for passing relative path to WSL binaries)
    - To write a literal `\`, you must escape it with another leading `\`: so it will be `\\`
  - *Ex)*
    - *Input*: `printf \example\\\\strin\\g:\\ \\'[%%s]\\' "ARG-INPUT"`
    - *(After conversion by the script)*: `printf /example\\strin\g:\ \'[%s]\' "ARG-INPUT"` (<- actual input on WSL shell)
    - *Output*: `/example\string: '[ARG-INPUT]'`
- Of course, special characters of cmd/powershell themselves should be also escaped
-->