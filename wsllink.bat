@echo off







:::::: MAIN ROUTINE ::::::

setlocal EnableDelayedExpansion

:: binary name
set SCRIPTNAME="wsllink"
set WSLCMDLINE="%~n0"


:: branch modes
if not %WSLCMDLINE% == %SCRIPTNAME% (
  :: execution mode
  call :execution-mode %%*
) else (
  :: management mode
  call :management-mode %%*
)

exit /b 0







:::::: EXECUTION MODE ::::::



:execution-mode

  :: check GUI exec
  set GUIARG=
  if %WSLCMDLINE:~1,1% == . (
    set WSLCMDLINE="!WSLCMDLINE:~2,-1!"
    set GUIARG="tmux" "new" "-d"
  )

  :: append args to cmdline
  for %%G in (%*) do (call :execution-mode_append-arg %%G)

  :: execute cmdline
  wsl -- . /etc/profile; . $HOME/.profile; %GUIARG% %WSLCMDLINE%
  :: echo %WSLCMDLINE%

  exit /b 0



:execution-mode_append-arg

  set ARG=%*
  
  :: convert all \ to / (for relative path args)
  call :execution-mode_append-arg_slash %%*
  :: remove all doublequotes for test
  set ARGNOQUOTE=%ARG:"=%
  ::"
  :: extract first 3 chars
  set ARGHEAD=%ARGNOQUOTE:~0,3%
  :: check if starts with drive pattern (absolute path arg)
  set ARGHEAD | findstr /R /C:"[a-zA-Z]:/" >nul

  :: append arg
  if not ERRORLEVEL 1 (
    :: if windows absolute path
    set WSLCMDLINE=!WSLCMDLINE! "^$^(/bin/wslpath %ARG%^)"
  ) else (
    :: if relative path, options, etc.
    set WSLCMDLINE=!WSLCMDLINE! %ARG%
  )


  exit /b 0




:execution-mode_append-arg_slash

  set ARG=%*
  
  :: encode {, }, \\ into sequence of {, }
  set ARG=%ARG:{={{{%
  set ARG=%ARG:}={{}%
  set ARG=%ARG:\\={}}%

  :: convert remaining \ (non-escaped back slash) to /
  set ARG=%ARG:\=/%

  :: recover {, }, \\ (change \\ to \)
  set ARG=%ARG:{}}=\%
  set ARG=%ARG:{{}=}%
  set ARG=%ARG:{{{={%

  exit /b 0






:::::: MANAGEMENT MODE ::::::



:management-mode

  :: trim all doublequotes for %1, to prevent error
  set OP=%~1

  :: branches
  if "%OP%" == "new" (
    call :management-mode_new %%*
  ) else if "%OP%" == "add" (
    call :management-mode_new %%*
  ) else if "%OP%" == "ln" (
    call :management-mode_new %%*
  ) else if "%OP%" == "del" (
    call :management-mode_del %%*
  ) else if "%OP%" == "rm" (
    call :management-mode_del %%*
  ) else if "%OP%" == "list" (
    call :management-mode_list
  ) else (
    call :management-mode_help
  )

  exit /b 0



:management-mode_new

  :: arg check
  set CMDNAME=%~2
  if not defined CMDNAME (
    echo usage: %~n0 %~1 [linux-command-to-link]
    exit /b 0
  )
  if "%CMDNAME%" == %SCRIPTNAME% (
    echo %~n0: ERROR: '%CMDNAME%' is invalid.
    exit /b 0
  )
  if exist "%~dp0%CMDNAME%.bat" (
    echo %~n0: ERROR: '%CMDNAME%.bat' already exists.
    exit /b 0
  )


  :: set GUI exec flag
  set GUIEXEC=
  if %CMDNAME:~0,1% == . (
    set GUIEXEC=1
  )


  :: create new symlink
  mklink "%~dp0%CMDNAME%.bat" "%~n0.bat" || (
    echo %~n0: ERROR: Failed to create a command symlink '%CMDNAME%.bat'.
    echo                 Please check if you either enabled 'Developer Mode' on Windows,
    echo                 or executed the command with admin privilege.
  )


  :: create new symlink for GUI (execute on explorer)
  if defined GUIEXEC (
    if not exist "%~dp0%CMDNAME%.cmd" (
      mklink "%~dp0%CMDNAME%.cmd" "%CMDNAME%.bat" || (
        echo %~n0: ERROR: Failed to create a command symlink '%CMDNAME%.cmd'.
        echo                 Please check if you either enabled 'Developer Mode' on Windows,
        echo                 or executed the command with admin privilege.
      )
    )
  )

  :: print result symlink list
  call :management-mode_list


  exit /b 0



:management-mode_del

  :: arg check
  set CMDNAME=%~2
  if not defined CMDNAME (
    echo usage: %~n0 %~n1 [linux-command-to-delete]
    call :management-mode_list
    exit /b 0
  )
  if "%CMDNAME%" == %SCRIPTNAME% (
    echo %~n0: ERROR: '%CMDNAME%' is invalid.
    exit /b 0
  )
  if not exist "%~dp0%CMDNAME%.bat" (
    echo %~n0: ERROR: '%CMDNAME%.bat' does not exist.
    exit /b 0
  )


  :: set GUI exec flag
  set GUIEXEC=
  if %CMDNAME:~0,1% == . (
    set GUIEXEC=1
  )
  

  :: delete existing symlink
  del "%~dp0%CMDNAME%.bat" || (
    echo %~n0: ERROR: Failed to delete a command symlink '%CMDNAME%.bat'.
    echo                 Please check if you have enough privilege to delete.
  )


  :: delete existing symlink for GUI (execute on explorer)
  if defined GUIEXEC (
    if exist "%~dp0%CMDNAME%.cmd" (
      del "%~dp0%CMDNAME%.cmd" || (
        echo %~n0: ERROR: Failed to delete a command symlink '%CMDNAME%.cmd'.
        echo                 Please check if you have enough privilege to delete.
      )
    )
  )

  :: print result symlink list
  call :management-mode_list


  exit /b 0



:management-mode_list

  :: pattern string
  set LINK_PATH=%~n0.bat
  set LINK_PATH=\[%LINK_PATH:\=\\%\]

  :: build symlink list
  set LINK_LIST=[Symlink-List]
  for /F "tokens=2 delims=>[" %%G in ('dir /AL %~dp0 ^| findstr /E /C:"%LINK_PATH%"') do (
    set LINK_LIST=!LINK_LIST! %%G

    :: trim the extension if there is
    if "!LINK_LIST:~-5,-1!" == ".bat" (
      set LINK_LIST=!LINK_LIST:~0,-5!
    )
  )

  echo.
  echo %LINK_LIST%

  exit /b 0



:management-mode_help

  :: help msg
  echo usage: %~n0 ^<operation^> [^<arg^>]
  echo.
  echo  ^<operation^>
  echo.
  echo    - Create a new command link
  echo.
  echo        %~n0 new ^<command-name^>
  echo        %~n0 add ^<command-name^>
  echo        %~n0 ln ^<command-name^>
  echo.
  echo    - Delete an existing command link
  echo.
  echo        %~n0 del ^<command-name^>
  echo        %~n0 rm ^<command-name^>
  echo.
  echo    - List all existing command links
  echo.
  echo        %~n0 list
  echo.


  exit /b 0


