1. Download and Extract python embeded version to `./python`  
2. Modify `python*._pth`, uncomment `import site`  
3. Set `./python/` as `PATH` environment Variable  
4. Change directory to `./python`, download [get-pip.py](https://bootstrap.pypa.io/get-pip.py) and run `python get-pip.py`  
5. Add `./python/Scripts/` in `PATH` so that you can install pip packages  


## Add tkinter  
1. Copy `tcl` folder from a regular installation root folder;  
2. Copy `Lib\tkinter` folder from the regular installation root folder;  
3. Copy `DLLs\_tkinter.pyd`,`DLLs\tcl86t.dll`,`DLLs\tk86t.dll` from the regular installation root folder;  