@echo off
setlocal EnableDelayedExpansion

:::::: MAIN ROUTINE ::::::

:: binary name
set WSLCMDLINE="%~n0"
if not %WSLCMDLINE% == "wslbrdg" (
  :: execution mode
  call :execution-mode %*
) else (
  :: management mode
  call :management-mode %*
)

exit /b 0







:::::: EXECUTION MODE ::::::


:execution-mode

  :: append args to cmdline
  for %%G in (%*) do (call :execution-mode_append-arg %%G)

  :: execute cmdline
  wsl -- . /etc/profile; . $HOME/.profile; %WSLCMDLINE%
  ::echo %WSLCMDLINE%

  exit /b 0


:execution-mode_append-arg
  set ARG=%*
  :: convert all \ to / (for relative path args)
  set ARG=%ARG:\=/%

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







:::::: MANAGEMENT MODE ::::::


:management-mode

  :: trim all doublequotes for %1, to prevent error
  set OP="%1"
  set OP=%OP:"=%
  ::"

  :: branches
  if "%OP%" == "new" (
    call :management-mode_new %2
  ) else if "%OP%" == "del" (
    echo DELETE OP
  ) else if "%OP%" == "list" (
    call :management-mode_list
  ) else (
    echo HELP OP
  )
    
  exit /b 0


:management-mode_new
  
  mklink "%~dp0%~n1.bat" "%~dp0%~n0.bat" || (
    echo %~n0: ERROR: Failed to creating a command symlink '%~n1'.
    echo                 Please check if you either enabled 'Developer Mode' on Windows,
    echo                 or executed the command with admin privilege.
  )


  exit /b 0


:management-mode_list

  :: pattern string
  set LINK_PATH=%~dp0%~n0.bat
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

  echo %LINK_LIST%

  exit /b 0



