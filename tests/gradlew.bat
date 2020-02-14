@echo "This is gradlew. You made it!"

@echo "arg %2%"
@echo "arg %3%"

@if "%1%" EQU "fail" (
    echo "it failed"
    exit /b 2
)
