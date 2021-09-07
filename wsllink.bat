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





:: (%*: script-args)
:execution-mode
  setlocal



  call :parse-cmdname "%~n0" CMDNAME USERNAME GUIEXEC


  :: build base cmdline
  set USERARG=
  if defined USERNAME (
    set USERARG=-u %USERNAME%
  )
  set GUIARG=
  if defined GUIEXEC (
    set GUIARG="tmux" "new" "-d"
  )
  set WSLCMDLINE=%CMDNAME%


  :: append args to cmdline
  for %%G in (%*) do (call :execution-mode_append-arg %%G WSLCMDLINE)


  :: execute cmdline
  wsl %USERARG% -- . /etc/profile; . $HOME/.profile; %GUIARG% %WSLCMDLINE%
  ::echo %WSLCMDLINE%



  endlocal
  exit /b 0





:: (%1: input-arg) (%2: full-wsl-cmdline (Input/Output))
:execution-mode_append-arg
  setlocal



  set ARG=%1
  set WSLCMDLINE=!%2!
  
  :: convert all \ to / (for relative path args)
  call :execution-mode_append-arg_slash ARG
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



  endlocal & set %2=%WSLCMDLINE%
  exit /b 0





:: (%1: input (Input/Output))
:execution-mode_append-arg_slash
  setlocal



  :: args
  set INPUT=!%1!
  
  
  :: encode {, }, \\ into sequence of {, }
  set INPUT=%INPUT:{={{{%
  set INPUT=%INPUT:}={{}%
  set INPUT=%INPUT:\\={}}%

  :: convert remaining \ (non-escaped back slash) to /
  set INPUT=%INPUT:\=/%

  :: recover {, }, \\ (change \\ to \)
  set INPUT=%INPUT:{}}=\%
  set INPUT=%INPUT:{{}=}%
  set INPUT=%INPUT:{{{={%



  endlocal & set %1=%INPUT%
  exit /b 0






:::::: MANAGEMENT MODE ::::::





:: (%1: op_mode) (%2: arg-1) (%3: arg-2) ...
:management-mode
  setlocal



  :: trim all doublequotes for %1, to prevent error
  set OP=%~1
  set OP_BRANCH=

  if "%OP%" == "new" set OP_BRANCH=n
  if "%OP%" == "add" set OP_BRANCH=n
  if "%OP%" == "ln" set OP_BRANCH=n
  if "%OP%" == "del" set OP_BRANCH=d
  if "%OP%" == "rm" set OP_BRANCH=d
  if "%OP%" == "list" set OP_BRANCH=l

::echo "%~1" "%~2" "%~3"

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
  


  endlocal
  exit /b 0





:: (%1: command-name-1) (%2: command-name-2) ...
:management-mode_new
  setlocal



  :: parse command name & user name from script name
  call :parse-cmdname %%~1 CMDNAME USERNAME GUIEXEC
  set CMDNAME_WITHUSER=%~1


  :: create new link
  set ERROR=0
  set CMDNAME_UNQ=%CMDNAME%
  call :unquote CMDNAME_UNQ
  if "%CMDNAME_UNQ%" == %SCRIPTNAME% (
    set ERROR=1
  ) else if exist "%~dp0%CMDNAME_WITHUSER%.bat" (
    set ERROR=2
  ) else (


    mklink "%~dp0%CMDNAME_WITHUSER%.bat" "%~n0.bat" >nul 2>nul && (

      if defined GUIEXEC (
    
        if not exist "%~dp0%CMDNAME_WITHUSER%.cmd" (
      
          mklink "%~dp0%CMDNAME_WITHUSER%.cmd" "%CMDNAME_WITHUSER%.bat" >nul 2>nul || (
            set ERROR=3
          )
      
        ) else (
          set ERROR=4
          del "%~dp0%CMDNAME_WITHUSER%.bat" >nul 2>nul
        )
      
      )

    ) || (
      set ERROR=3
    )
  )


  :: print error
  if "%ERROR%" == "1" (
    echo %~n0: ERROR: '%CMDNAME_WITHUSER%' is invalid.
  ) else if "%ERROR%" == "2" (
    echo %~n0: ERROR: Command '%CMDNAME_WITHUSER%' already exists.
  ) else if "%ERROR%" == "3" (
    echo %~n0: ERROR: Failed to link a command '%CMDNAME_WITHUSER%'.
    echo                 Please check if you either enabled 'Developer Mode' on Windows,
    echo                 or executed the command with admin privilege.
  ) else if "%ERROR%" == "4" (
    echo %~n0: ERROR: Failed to link a command '%CMDNAME_WITHUSER%'.
    echo                 There is unknown existing file '%~dp0%CMDNAME_WITHUSER%.cmd'.
    echo                 Delete the file manually and try again.
  ) else (
    echo  - Linked command '%CMDNAME_WITHUSER%' to WSL.
  )



  endlocal
  exit /b 0





:: (%1: command-name-1) (%2: command-name-2) ...
:management-mode_del
  setlocal



  :: parse command name & user name from script name
  call :parse-cmdname %%~1 CMDNAME USERNAME GUIEXEC
  set CMDNAME_WITHUSER=%~1


  :: create new link
  set ERROR=0
  set CMDNAME_UNQ=%CMDNAME%
  call :unquote CMDNAME_UNQ
  if "%CMDNAME_UNQ%" == %SCRIPTNAME% (
    set ERROR=1
  ) else if not exist "%~dp0%CMDNAME_WITHUSER%.bat" (
    set ERROR=2
  ) else (


    :: delete existing symlink
    del "%~dp0%CMDNAME_WITHUSER%.bat" >nul 2>nul && (


      if defined GUIEXEC (
      
        if exist "%~dp0%CMDNAME_WITHUSER%.cmd" (
        
          del "%~dp0%CMDNAME_WITHUSER%.cmd" >nul 2>nul || (
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
    echo %~n0: ERROR: '%CMDNAME_WITHUSER%' is invalid.
  ) else if "%ERROR%" == "2" (
    echo %~n0: ERROR: Command '%CMDNAME_WITHUSER%' does not exist.
  ) else if "%ERROR%" == "3" (
    echo %~n0: ERROR: Failed to delete a command '%CMDNAME_WITHUSER%'.
    echo                 Please check if you have enough privilege to delete.
  ) else (
    echo  - Unlinked command '%CMDNAME_WITHUSER%' from WSL.
  )



  endlocal
  exit /b 0





:management-mode_list
  setlocal
  


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



  endlocal
  exit /b 0





:management-mode_help
  setlocal



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



  endlocal
  exit /b 0





:: (%1: input) (%2: command-name (OUT)) (%3: user-name (OUT)) (%4: is-guiexec (OUT))
:parse-cmdname
  setlocal



  :: args
  set CMDFULL=%1
  set CMDNAME=
  set USERNAME=
  set GUIEXEC=

  :: encode {, }, @@ into sequence of {, }
  set CMDFULL=%CMDFULL:{={{{%
  set CMDFULL=%CMDFULL:}={{}%
  set CMDFULL=%CMDFULL:@@={}}%

  :: split string with @ (CMDNAME@USERNAME)
  set "CMDNAME=%CMDFULL:@=" & set "USERNAME=%"

  :: recover {, }, @@ (change @@ to @)
  set CMDNAME=%CMDNAME:{}}=@%
  set CMDNAME=%CMDNAME:{{}=}%
  set CMDNAME=%CMDNAME:{{{={%

  if defined USERNAME (
    set USERNAME=%USERNAME:{}}=@%
    set USERNAME=%USERNAME:{{}=}%
    set USERNAME=%USERNAME:{{{={%
  )

  set CMDNAME_UNQ=%CMDNAME%
  call :unquote CMDNAME_UNQ

  :: set GUI exec flag
  if %CMDNAME_UNQ:~0,1% == . (
    set GUIEXEC=1
  )



  endlocal & set %2=%CMDNAME%& set %3=%USERNAME%& set %4=%GUIEXEC%
  exit /b 0





:unquote
for /f "delims=" %%G in ('echo %%%1%%') do set %1=%%~G
