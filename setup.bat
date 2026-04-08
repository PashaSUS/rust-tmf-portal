@echo off
setlocal enabledelayedexpansion

echo ============================================
echo   TMF Portal - Setup
echo ============================================
echo.

:: Check Docker
where docker >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Docker is not installed or not in PATH.
    echo         Install Docker Desktop: https://www.docker.com/products/docker-desktop
    exit /b 1
)

docker info >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Docker daemon is not running. Start Docker Desktop first.
    exit /b 1
)

echo [OK] Docker found

:: Check docker compose
docker compose version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Docker Compose not available. Install Docker Desktop with Compose.
    exit /b 1
)

echo [OK] Docker Compose found
echo.

:: Always regenerate config.docker.json from config.json
echo [INFO] Generating config.docker.json from config.json...

powershell -NoProfile -Command "$c = Get-Content 'config.json' -Raw | ConvertFrom-Json; $c.portal.host = '0.0.0.0'; $c.portal.port = 4201; $c.portal.cors_origin = '*'; $c.seq.url = 'http://seq:80'; $j = $c | ConvertTo-Json -Depth 10; [System.IO.File]::WriteAllText('config.docker.json', $j, [System.Text.UTF8Encoding]::new($false))"

echo [OK] config.docker.json generated

echo.
echo ============================================
echo   Building and starting containers...
echo ============================================
echo.
echo   Portal (Frontend):  http://localhost:4200
echo   Backend API:        http://localhost:4201
echo   Swagger UI:         http://localhost:4201/swagger-ui/
echo   Seq Logs:           http://localhost:4202
echo.

docker compose -f docker-compose.yml build
if %errorlevel% neq 0 (
    echo [ERROR] Build failed.
    exit /b 1
)

docker compose -f docker-compose.yml up -d
if %errorlevel% neq 0 (
    echo [ERROR] Failed to start containers.
    exit /b 1
)

echo.
echo ============================================
echo   TMF Portal is running!
echo ============================================
echo.
echo   Portal:   http://localhost:4200
echo   API:      http://localhost:4201
echo   Swagger:  http://localhost:4201/swagger-ui/
echo   Seq:      http://localhost:4202
echo.
echo   Stop with:  docker compose down
echo   Logs with:  docker compose logs -f
echo.
