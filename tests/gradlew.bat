@echo "This is gradlew. You made it!"

@echo "arg 1 %1%"
@echo "arg 2 %2%"

@if "%1%" EQU "fail" (
    echo "it failed"
    exit /b 2
)
