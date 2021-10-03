一次基于`wry`的本地可执行程序模板，以`vue3`作为前端、`rust`作为后端的尝试。至于为什么不用`Electron`或者`tauri`：
  - `Electron`打包后的文件体积比较大，且资源文件不能集成到可执行程序；
  - `tauri`虽然功能丰富，但是学习起来不如更底层的实现容易。

项目名称源自唐代诗人刘禹锡的《乌衣巷》：`旧时王谢堂前燕 飞入寻常百姓家`。

## 调试与打包  

### 调试  
```bash
# 1. 启动vue（需要新开一个控制台）
cd web_src
yarn
yarn dev  # 此进程会阻塞

# 2. 运行cargo
cargo run
```

### 打包  
```bash
# 1. 打包web 页面
cd web_src  
yarn
yarn build  

# 2. 打包exe
cd ..  # 切换回根目录
cargo build --release
```

## Vue前端
1. 创建`vue3`项目：`yarn create vite web_src --template vue`；  
2. 在`vite.config.js`中修改输出目录；  
```js
// vite.config.js
export default defineConfig({
  // ... ,
  build: {
    outDir: '../web',  // 打包结果输出目录
  },
  server: {
    port: 8341,        // 指定调试服务的的各种信息
  }
})
```
3. 项目有一些缺点，当然也有一个优点：由于是调用系统的`webview`功能，打包后的可执行文件体积非常小！
    - `vscode`的插件不能正确识别`web_srv`子项目，所以最好能在新的IDE 窗口修改前端部分。
    - 有些跨平台的代码需要手动添加宏操作，例如图标等资源文件打包、窗口管理等

## Rust后端
1. 以`wry`作为后端：`wry = "0.12.2"`；  
2. 将`web`作为二进制文件包含至`exe`：`include_dir = "0.6.2"`；
3. `webview`不能直接在前端获取本地文件路径，所以需要`rfd`来从后端选择并处理文件；
4. 根据文件后缀名获取`MIME`类型：`mime = "0.3.16"`；  


### 条件编译  

1. 调试状态
    - `#[cfg(debug_assertions)]`：判断程序是否处于debug 状态  
    - `#[cfg(not(debug_assertions))]`：可以用于任何语句（块）前面  
2. 取消Windows 下的命令行窗口：
    - 在`main.rs`最上方添加：`#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`