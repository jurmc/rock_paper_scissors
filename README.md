Build raylib-rs on Windows:
1. Install Visual Studio
2. Install Clang tools in Visual Studio Installer
3. In PowerShell:
   $env:LIBCLANG_PATH = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\bin'
   $env:CMAKE_INCLUDE_PATH = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib\clang\19\include'

