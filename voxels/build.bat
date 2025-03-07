@echo off

cls
echo + build +

set arg1=%1
if "%arg1%"=="" set arg1=run

sokol-shdc.exe -i ./src/shaders.glsl -o ./src/shaders.rs --slang hlsl5:wgsl:glsl430 -f sokol_rust

cargo %arg1%

