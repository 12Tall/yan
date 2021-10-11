# 燕/yan  

`旧时王谢堂前燕 飞入寻常百姓家`——《乌衣巷》唐·刘禹锡。  

此项目是一个基于`rust`和`webview`的模板。后端采用`wry`，前端采用`vue3`作为骨架，前后端通过IPC 进程通信。至于为什么不直接用`Electron`或者`tauri`？首先`Electron`打包后的文件体积比较大，但是打包后的结果文件不够简洁；而`tauri`由于封装程度比较高，并不适合拿来学习理解整个项目的实现原理。

当然，通过`webview(2)`也有一些瑕疵，即无法直接调用平台的某些api、需要自己设置`user_data_directory`的位置。这些都是值得的，因为这些问题即使这次不遇到，以后也可能会遇到。就像《天月神话》里面说的，根本就没有银弹。

-----

## 说明  
由于没有嵌入`Chromium`，编译后的结果文件会比较小；但是这也导致要求系统必须已经预装了`Webview`运行时。当然也可以通过`set_env`来指定固定版本的`webview(2)`运行时，但是这样的编译结果就违背了简洁的初衷。另外，某些跨平台的代码，需要添加宏命令来实现条件编译。

### Rust 后端
1. `Object`对象可以通过`get`方法访问键值；
2. 修改图标：需要引用`ico`库，详细代码参见[wry/examples/icon.rs](https://github.com/tauri-apps/wry/blob/6ffd1d7194bda9ca1434fa2ca0d0bd0c8237f01f/examples/icon.rs)，需要注意的是源代码没有包含在`wry`库中，且有些类型已重新命名。
3. 条件编译
  - `#[cfg(debug_assertions)]`：判断程序是否处于debug 状态，可用于任何语句块之前
  - `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`，添加在`main.rs`最前面


### Vue前端
1. `vscode`的插件不能正确识别`web_srv`子项目，所以最好能在新的IDE 窗口修改前端部分；
2. 前端事件的监听器可以是异步函数；


## 依赖的库  
不必过于担心引入过多的库而导致程序运行效率低下，因为：
  1. 事情总要做，不是自己做就是别人做；
  2. 编译器会帮我们优化；
  3. 开发效率也很重要，并且引入三方库让代码更整洁。

### rust 库  

序号|名称|说明  
:---:|:---|:---  
01|Geal/nom|二进制文件解析  
02|WLBF/single-instance|跨平台应用单例模式  
03|BurntSushi/byteorder|Read trait 扩展，可用于二进制文件解析，比nom 更基础  
04|tauri-app/wry|基于tao，支持webview2，生成桌面应用  
05|Michael-F-Bryan/include_dir|将文件夹以二进制的形式打包进exe
06|PolyMeilex/rfd|跨平台的对话框库  
07|hyperium/mime|MIME 文件类型支持  
08|retep998/winapi-rs|win32api 的封装
09|microsoft/windows-rs|微软官方库，但有些东西不全  
10|mdsteele/rust-ico|ico 图标文件支持

### js 库  

序号|名称|说明  
:---:|:---|:---  
01|vue3|前端框架
02|jiggzson/nerdamer|数学符号计算库


## 调试与打包  
### 调试  
调试模式下，需要启动一个vue 的web 服务，可以实现前端代码修改的实时同步。
```bash
# 1. 启动vue（需要新开一个控制台）
cd web_src
yarn
yarn dev  # 此进程会阻塞

# 2. 运行cargo
cargo run
```

### 发布  
发布模式下，需要先编译前端vue 代码，然后再编译rust 代码。
```bash
# 1. 打包web 页面
cd web_src  
yarn
yarn build  

# 2. 打包exe
cd ..  # 切换回根目录
cargo build --release
```