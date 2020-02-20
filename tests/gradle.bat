@echo "This is global gradle. You made it!"

@echo "arg 1 %1%"
@echo "arg 2 %2%"

@for %%I in (.) do @set CWD=%%~nxI
@echo "cwd %CWD%"

@if "%1%" EQU "fail" (
    echo "it failed"
    exit /b 2
)
