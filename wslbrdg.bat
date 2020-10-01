@echo off
setlocal EnableDelayedExpansion

:::::: MAIN ROUTINE ::::::

:: binary name
set WSLCMDLINE=%~n0

if not "%WSLCMDLINE%" == "wslbrdg" (
    :: Execution Mode
  call :execution-mode %*
) else (
  :: Management Mode
  call :management-mode %*
)

exit /b 0







:::::: EXECUTION MODE ::::::


:execution-mode

  :: append args to cmdline
  for %%G in (%*) do (call :execution-mode_append-arg %%G)

  :: execute cmdline
  wsl -- . /etc/profile; . $HOME/.profile; %WSLCMDLINE%

  exit /b 0


:execution-mode_append-arg
  set ARG=%*
  :: convert all \ to / (for relative path args)
  set ARG=%ARG:\=/%
  :: escape all doublequotes
  set ARG=%ARG:"=\"%

  :: remove all doublequotes
  set ARGNOQUOTE=%ARG:\"=%
  :: extract first 3 chars
  set ARGHEAD=%ARGNOQUOTE:~0,3%
  :: check if starts with drive pattern (absolute path arg)
  set ARGHEAD | findstr /R /C:"[a-zA-Z]:/" >nul

  :: append arg
  if not ERRORLEVEL 1 (
    :: if windows absolute path
    set WSLCMDLINE=!WSLCMDLINE! ^$^(/bin/wslpath %ARG%^)
  ) else (
    :: if relative path, options, etc.
    set WSLCMDLINE=!WSLCMDLINE! %ARG%
  )

  
  exit /b 0







:::::: MANAGEMENT MODE ::::::


:management-mode

  if "%1" == "new" (
    call :management-mode_new %2
  ) else if "%1" == "del" (
    echo DELETE OP
  ) else if "%1" == "list" (
    echo LIST OP
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




