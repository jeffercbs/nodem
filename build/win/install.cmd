@echo off

set /P NODEM_PATH=""
set NODEM_HOME=%NODEM_PATH%
set NODEM_SYMLINK=C:\Program Files\nodejs
setx /M NODENM_HOME "%NODEM_HOME%"
setx /M NODEM_SYMLINK "%NODEM_SYMLINK%"

@echo on