@echo "This is gradlew. You made it!"

@echo "arg 1 %2%"
@echo "arg 2 %3%"

@if "%1%" EQU "fail" (
    echo "it failed"
    exit /b 2
)
