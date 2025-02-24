@echo off

set arg1=%1
if "%arg1%"=="" set arg1=run

sokol-shdc.exe -i ./src/shader.glsl -o ./src/shader.rs --slang hlsl5:wgsl:glsl430 -f sokol_rust

cargo %arg1%

