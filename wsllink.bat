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
  set OP_BRANCH=

  if "%OP%" == "new" set OP_BRANCH=n
  if "%OP%" == "add" set OP_BRANCH=n
  if "%OP%" == "ln" set OP_BRANCH=n
  if "%OP%" == "del" set OP_BRANCH=d
  if "%OP%" == "rm" set OP_BRANCH=d
  if "%OP%" == "list" set OP_BRANCH=l



  if defined OP_BRANCH (


    :: if mod mode (iterate for all args)
    if not "%OP_BRANCH%" == "l" (

      set ARG=%~2
      if not defined ARG (
        echo usage: %~n0 %~1 [command-name-1] [command-name-2] ...
        exit /b 0
      )
    
      :: iterate for all arguments except the first one
      set ARG1_PASSED=
      for %%G in (%*) do (
        if defined ARG1_PASSED (
          if "%OP_BRANCH%" == "n" ( call :management-mode_new %%G ) ^
          else if "%OP_BRANCH%" == "d" ( call :management-mode_del %%G )
        ) else (
          set ARG1_PASSED=1
        )
      )
      echo.
    )


    :: print result symlink list
    call :management-mode_list


  ) else (
  
    call :management-mode_help
    
  )
  

  exit /b 0



:management-mode_new

  set CMDNAME=%~1
  
  :: set GUI exec flag
  set GUIEXEC=
  if %CMDNAME:~0,1% == . (
    set GUIEXEC=1
  )


  :: error flag
  set ERROR=0


  :: create new link
  if "%CMDNAME%" == %SCRIPTNAME% (
    set ERROR=1
  ) else if exist "%~dp0%CMDNAME%.bat" (
    set ERROR=2
  ) else (



    mklink "%~dp0%CMDNAME%.bat" "%~n0.bat" >nul 2>nul && (


      if defined GUIEXEC (
    
        if not exist "%~dp0%CMDNAME%.cmd" (
      
          mklink "%~dp0%CMDNAME%.cmd" "%CMDNAME%.bat" >nul 2>nul || (
            set ERROR=3
          )
      
        ) else (
          set ERROR=4
          del "%~dp0%CMDNAME%.bat" >nul 2>nul
        )
      
      )


    ) || (
      set ERROR=3
    )
  )



  :: print error
  if "%ERROR%" == "1" (
    echo %~n0: ERROR: '%CMDNAME%' is invalid.
  ) else if "%ERROR%" == "2" (
    echo %~n0: ERROR: Command '%CMDNAME%' already exists.
  ) else if "%ERROR%" == "3" (
    echo %~n0: ERROR: Failed to link a command '%CMDNAME%'.
    echo                 Please check if you either enabled 'Developer Mode' on Windows,
    echo                 or executed the command with admin privilege.
  ) else if "%ERROR%" == "4" (
    echo %~n0: ERROR: Failed to link a command '%CMDNAME%'.
    echo                 There is unknown existing file '%~dp0%CMDNAME%.cmd'.
    echo                 Delete the file manually and try again.
  ) else (
    echo  - Linked command '%CMDNAME%' to WSL.
  )

  exit /b 0



:management-mode_del


  set CMDNAME=%~1
  
  :: set GUI exec flag
  set GUIEXEC=
  if %CMDNAME:~0,1% == . (
    set GUIEXEC=1
  )


  :: error flag
  set ERROR=0


  :: create new link
  if "%CMDNAME%" == %SCRIPTNAME% (
    set ERROR=1
  ) else if not exist "%~dp0%CMDNAME%.bat" (
    set ERROR=2
  ) else (


    :: delete existing symlink
    del "%~dp0%CMDNAME%.bat" >nul 2>nul && (


      if defined GUIEXEC (
      
        if exist "%~dp0%CMDNAME%.cmd" (
        
          del "%~dp0%CMDNAME%.cmd" >nul 2>nul || (
            set ERROR=3
          )
          
        )
      )


    ) || (
      set ERROR=3
    )


  )



  :: print error
  if "%ERROR%" == "1" (
    echo %~n0: ERROR: '%CMDNAME%' is invalid.
  ) else if "%ERROR%" == "2" (
    echo %~n0: ERROR: Command '%CMDNAME%' does not exist.
  ) else if "%ERROR%" == "3" (
    echo %~n0: ERROR: Failed to delete a command '%CMDNAME%'.
    echo                 Please check if you have enough privilege to delete.
  ) else (
    echo  - Unlinked command '%CMDNAME%' from WSL.
  )

  exit /b 0



:management-mode_list

  :: pattern string
  set LINK_PATH=%~n0.bat
  set LINK_PATH=\[%LINK_PATH:\=\\%\]

  :: build symlink list
  set LINK_LIST=
  for /F "tokens=2 delims=>[" %%G in ('dir /AL %~dp0^*.bat 2^>nul ^| findstr /E /C:"%LINK_PATH%"') do (
    set LINK_LIST=!LINK_LIST! %%G

    :: trim the extension if there is
    if "!LINK_LIST:~-5,-1!" == ".bat" (
      set LINK_LIST=!LINK_LIST:~0,-5!
    )
  )

  if defined LINK_LIST (
    echo [Command-List] %LINK_LIST%
  ) else (
    echo ^(Command-List has no entry^)
  )

  exit /b 0



:management-mode_help

  :: help msg
  echo usage: %~n0 ^<operation^> [^<arg1^> ^<arg2^> ...]
  echo.
  echo  ^<operation^>
  echo.
  echo    - Link new commands
  echo.
  echo        %~n0 new ^<command-name-1^> ^<command-name-2^> ...
  echo        %~n0 add ^<command-name-1^> ^<command-name-2^> ...
  echo        %~n0 ln ^<command-name-1^> ^<command-name-2^> ...
  echo.
  echo    - Unlink existing commands
  echo.
  echo        %~n0 del ^<command-name-1^> ^<command-name-2^> ...
  echo        %~n0 rm ^<command-name-1^> ^<command-name-2^> ...
  echo.
  echo    - List linked commands
  echo.
  echo        %~n0 list
  echo.


  exit /b 0


