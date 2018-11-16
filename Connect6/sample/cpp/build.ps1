mkdir build
pushd build
cmake .. -DCMAKE_GENERATOR_PLATFORM=x64
msbuild .\sample_cpp.sln /p:configuration=release /p:platform=x64 /p:machine=x64
popd