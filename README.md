# WslLink
- A simple executable binary for running commands/programs of WSL at Windows side (cmd, powershell, icon-click, etc.)
- Author: snoopy3476@outlook.com


## Description


### About
- Create and manage symlinks to commands of WSL (inside the directory where the executable binary exists)
- When running WSL commands on Windows shell, converts Windows path arguments to WSL path if exists.

  See [Path argument auto-conversion part](#path-argument-auto-conversion-and-backslash-escaping) for more details.



### Brief usage examples

- Use WSL commands directly on Windows command prompt
```
C:\>wsllink list
(No linked WSL command)
 
C:\>git --version & ls C:\Users
'git' is not recognized as an internal or external command,
operable program or batch file.
'ls' is not recognized as an internal or external command,
operable program or batch file.

C:\>wsllink add git ls
 - Linked command(s) successfully

C:\>wsllink list
git     ls

C:\>git --version & ls C:\Users
git version 2.25.1
'All Users'   Default  'Default User'   Public   desktop.ini   snoop

C:\>
```

- Set a WSL program as a default program for some file-extensions
  - When clicking a file with those file-extensions on Windows explorer, the file can be open with the WSL program directly
  - See [Detached process mode (GUI program mode) part](#detached-process-mode-gui-program-mode) for more information


## Prerequisites
- [WSL Installed](https://docs.microsoft.com/en-us/windows/wsl/install)
- Windows 'Developer Mode' enabled, for creating symlink in Windows
  - This is optional, but [creating new commands in the command management](#executable-basic-usage) below should be run as administrator if not in dev mode


## Build & Install

First, you can select one of below to install:

- Download a pre-compiled executable binary from [Releases](https://github.com/snoopy3476/WslLink/releases).
  1. Make folder for WslLink
  
     (CMD/PowerShell) `cmd /C "mkdir %USERPROFILE%\WslLink\bin"`
     
  2. Put the downloaded binary into the created folder
  
     (CMD/PowerShell) `cmd /C "copy path\to\wsllink.exe %USERPROFILE%\WslLink\bin"`

- Build on Windows native (may need to install Visual Studio)
  1. [Install Rust for Windows](https://www.rust-lang.org/tools/install)
  2. Build and install with Cargo in Windows CMD:
  
     (CMD/PowerShell) ```cmd /C "cargo install --git=https://github.com/snoopy3476/WslLink.git --root=%USERPROFILE%\WslLink"```
     
     Executable will be placed in 'bin' folder inside the specified root.
    
- Build on WSL (no need to install Visual Studio, but mingw-64 is needed)
  1. [Install Rust for WSL](https://www.rust-lang.org/tools/install)
  2. Install prerequisites on WSL
     - Install mingw-64

       (Debian/Ubuntu Shell Example) ```sudo apt install mingw-w64```
       
     - Install rustup target for cross-compile
     
       (WSL Shell) ```rustup target add x86_64-pc-windows-gnu```
       
  3. Build on WSL & install to Windows with Cargo:
  
     (WSL Shell)
     ```
     # Get %USERPROFILE% env var from Windows
     WSLLINK_ROOT=$(
         wslpath $(cmd.exe /Q /C "echo %USERPROFILE%\\WslLink" \
                   2>/dev/null | tr -d '\r' | tr -d '\n' )) &&

     # If getting $WSLLINK_ROOT is successful, cargo install to there
     test -n "$WSLLINK_ROOT" &&
     cargo install --git=https://github.com/snoopy3476/WslLink.git \
                   --target=x86_64-pc-windows-gnu \
                   --root="$WSLLINK_ROOT"
     
     # When failed to install because of abnormal $WSLLINK_ROOT,
     # run only 'cargo install ...' with setting '--root=' manually
     ```

     Executable will be placed in 'bin' directory inside the specified root.


Then, append the folder path (where the executable exists) to Windows 'PATH' environment variables:
   1. Append installed path to Windows PATH env var
      1. Go to System Properties
         1. Press `[Windows key] + [R]` keys
         2. Enter `SystemPropertiesAdvanced`
         3. Press `[Enter]` key
      2. Click 'Environment Variables'
      3. Double click 'Path' at **'User Environment Variable'** region, then add WslLink binary folder path into it
         - *Ex)* `%USERPROFILE%\WslLink\bin`
   2. Re-open CMD to apply modified PATH
      - If you are using Windows Terminal, you need to close and re-open Windows Terminal itself.


## Uninstall
1. Remove WslLink root folder
2. Remove WslLink bin folder from Windows PATH env vars
   1. Go to System Properties
      1. Press `[Windows key] + [R]` keys
      2. Enter `SystemPropertiesAdvanced`
      3. Press `[Enter]` key
   2. Click 'Environment Variables'
   3. Double click 'Path' at user environment variable region, find the WslLink path, and remove it
      - *Ex)* `%USERPROFILE%\WslLink\bin`


## Usage
### Executable basic usage
- Command management
  - Link new commands:
    ```
    wsllink add <command-1> (<command-2>) ...
            a        "            "       ...
            new      "            "       ...
            n        "            "       ...
    ```
  - Unlink existing commands:
    ```
    wsllink del <command-1> (<command-2>) ...
            d        "            "       ...
            rm       "            "       ...
            r        "            "       ...
    ```
  - List linked commands:
    ```
    wsllink list
            ls
            l
    ```
- Command execution
  - `<command-name> <command-arg1> <command-arg2> ...`
  - *Ex)*
    - `emacs ~/src/project1/project1.sh`           *(Using WSL path as arguments)*
    - `emacs C:\Users\(user-home)\bin\test.txt`    *(Using absolute Windows path as arguments)*
    - `emacs bin\test.txt`                         *(Using relative Windows path as arguments)*


### Detached process mode (GUI program mode)

- Note that 'Linux GUI server for Windows' (WSLg, VcXsrv, etc.) is required for running GUI programs!
- When creating a command, additional command with prepended `.` is also created internally

- Running a command starting with an `.` will run the command as a detached, backgroud process
  - Detached background process here does not tied to the running shell, so you can close the shell after running it
  - This is useful when you want to execute GUI program of WSL
  - *Ex)*
    - `wsllink new emacs`        *(Create new links to WSL command 'emacs')*
    - `.emacs bin\test.txt`      *(run 'emacs' at background, and detach)*

- Usage examples

  - **Open specific file-extensions with WSL GUI programs by default**
    - Set a WSL GUI program symlink (`.(command-name).exe`, executable with a leading dot) as a default program ('Open with...') for some file-extensions
      - After doing this, the WSL program can open a file with those extensions directly, if you double-click the file icon at Windows file explorer

  - **Add WSL GUI programs to Windows start menu as Windows program**
    - Make a shortcut link (.lnk) to the command file `(command-name).exe` (**NOT `.(command-name).exe` file! No leading dot here**) or run it directly, to run GUI programs with mouse click
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


### Command name format
By formatting command name when creating and executing a command, running WSL commands as different WSL user or distribution is also possible.

Formatting is done by the delimiter `!` (which has no problem to be executed as a command-name on cmd, powershell, and bash). Empty field will be set to default. Detailed full-format of the command name is as follows:
- `<command-name>!<user-name>!<dist-name>`
  - *Ex) Running command as...*
    - Default user & Default dist: `command`
    - User 'john' & Default dist: `command!john`
    - Default user & Dist 'Ubuntu': `command!!ubuntu`
    - User 'john' & Disk 'Debian': `command!john!debian`
- Usage *Ex)*
  ```
  C:\>wsllink l
  (No linked WSL command)

  C:\>wsllink a lsb_release!!ubuntu lsb_release!!debian
   - Linked command(s) successfully

  C:\>wsllink l
  lsb_release!!debian     lsb_release!!ubuntu

  C:\>lsb_release!!debian -d
  Description:    Debian GNU/Linux 11 (bullseye)

  C:\>lsb_release!!ubuntu -d
  Description:    Ubuntu 20.04.3 LTS
  
  C:\>
  
  ```


### Path argument auto-conversion and Backslash escaping
WslLink tries to convert Windows path arguments to WSL-understandable path. This is necessary because most Windows programs (including explorer.exe, etc.) pass path argument(s) as `\`-separated version, instead of `/` one. This function is disabled when the environment variable `WSLLINK_NO_ARGCONV` is set.

- Conversion of Windows absolute path to WSL path
  - Wraps `[a-zA-Z]:[\/]` patterned argument (after unquoted) with wslpath substitution
    - *Ex)*
      - *Input*: `printf "C:\tmp\example-file.txt"`
      - *Output*: `/mnt/c/tmp/example-file.txt`
  - Does not work if a Windows path is embedded inside other strings
    - *Ex)*
      - *Input*: `printf "Test=C:\tmp\example-file.txt"`
      - *Output*: `Test=C:/tmp/example-file.txt`

- Conversion of Windows relative path to WSL path
  - Unlike absolute path, as there is no reliable way to check whether the argument is relative or not, WslLink first converts all single `\` to `/` to cover almost all of relative path patterns.
  
    Then, to represent single or consecutive `\`(s), you can escape it with another prepended `\`. Detailed rules are as follows:
    - Rules
      - (Single `\`) -> `/`
        - This is for passing relative path to WSL binaries
      - (Consecutive `\`s) -> (Remaining `\`(s) without the first `\` character)
        - *Ex)*
          - `\\` -> `\`
          - `\\\` -> `\\`
          - ...
    - *Ex)*
      - *Input*: `printf \example\\\strin\\g:\\ \\'[%s]\\' "ARG-INPUT"`
      - *(After conversion is done before run)*: `printf /example\\strin\g:\ \'[%s]\' "ARG-INPUT"` (<- actual input on WSL shell)
      - *Output*: `/example\string: '[ARG-INPUT]'`
      
  - Of course, special characters of cmd/powershell themselves should be also escaped


### Environments
Following environment files are loaded before execution if exists:
- `/etc/profile`
- `$HOME/.profile`
- `(wsllink-exe-dir)\profile` (File `profile` inside the wsllink exe folder)