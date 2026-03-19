@echo off
chcp 65001 >nul
echo ========================================
echo   Tauri 应用测试环境检测工具
echo ========================================
echo.

:: 设置项目根目录
set PROJECT_ROOT=%~dp0..
cd /d %PROJECT_ROOT%

echo [1/5] 检查项目目录...
echo 当前目录: %CD%
echo.

echo [2/5] 查找 data-masker.exe...
echo.

:: 可能的路径列表
set PATHS[0]=windows-build\data-masker.exe
set PATHS[1]=src-tauri\target\release\data-masker.exe
set PATHS[2]=..\windows-build\data-masker.exe
set PATHS[3]=data-masker.exe

set FOUND=0
set EXE_PATH=

:: 遍历所有可能的路径
for /L %%i in (0,1,3) do (
    call set CURRENT_PATH=%%PATHS[%%i]%%
    if exist "!CURRENT_PATH!" (
        echo ✅ 找到: !CURRENT_PATH!
        set FOUND=1
        set EXE_PATH=!CURRENT_PATH!
    ) else (
        echo ❌ 未找到: !CURRENT_PATH!
    )
)

echo.

if "%FOUND%"=="1" (
    echo [3/5] 验证可执行文件...
    if exist "%EXE_PATH%" (
        echo ✅ 文件存在: %EXE_PATH%
        echo.

        echo [4/5] 检查文件大小...
        for %%A in ("%EXE_PATH%") do echo 文件大小: %%~zA 字节
        echo.

        echo [5/5] 设置环境变量...
        echo set EXE_PATH=%CD%\%EXE_PATH%
        echo.

        echo ========================================
        echo   ✅ 环境检测通过
        echo ========================================
        echo.
        echo 可执行文件路径:
        echo %CD%\%EXE_PATH%
        echo.
        echo 现在可以运行测试:
        echo   npm run test:e2e -- tests/e2e/tauri-desktop.spec.js
        echo.

    ) else (
        echo ❌ 文件不存在
    )
) else (
    echo [3/5] 未找到可执行文件
    echo.
    echo ========================================
    echo   ⚠️  需要准备 Tauri 应用
    echo ========================================
    echo.
    echo 解决方案:
    echo.
    echo 方案 1: 解压 windows-build.zip
    echo   1. 将 windows-build.zip 解压到项目根目录
    echo   2. 确保 windows-build\data-masker.exe 存在
    echo   3. 重新运行此脚本
    echo.
    echo 方案 2: 构建 Tauri 应用
    echo   1. npm run tauri build
    echo   2. 构建产物在 src-tauri\target\release\
    echo   3. 重新运行此脚本
    echo.
    echo 方案 3: 指定自定义路径
    echo   set EXE_PATH=完整路径\data-masker.exe
    echo   npm run test:e2e -- tests/e2e/tauri-desktop.spec.js
    echo.
)

pause
