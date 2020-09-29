@echo off
setlocal EnableDelayedExpansion

:::::: MAIN ROUTINE START ::::::


:: binary name
set WSLCMDLINE=%~n0

if "%WSLCMDLINE%"=="wslexec" (
  mklink "%~dp0%~n1.bat" "%~dp0%~n0.bat"
  exit /b 0
)

:: append args to cmdline
for %%G in (%*) do (call :appendarg %%G)

:: execute cmdline
bash -lc "%WSLCMDLINE%"

exit /b 0

:::::: MAIN ROUTINE END ::::::




:::::: SUB ROUTINE START ::::::

:appendarg
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
  set ARGHEAD|findstr /R /C:"[a-zA-Z]:/" >nul

  :: append arg
  if not ERRORLEVEL 1 (
    :: if windows absolute path
    set WSLCMDLINE=!WSLCMDLINE! ^$^(wslpath %ARG%^)
  ) else (
    :: if relative path, options, etc.
    set WSLCMDLINE=!WSLCMDLINE! %ARG%
  )

  
  exit /b 0

:::::: SUB ROUTINE END ::::::