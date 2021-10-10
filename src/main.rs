// use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
// use std::{
//     fs::File,
//     io::{self, BufReader, Read, Seek, SeekFrom},
//     mem
// };
// use std::env;
// // representation 描述、表示
// // packaged 表示紧密排列
// #[repr(C, packed)]
// #[derive(Debug)]
// struct DosHeader {
//     magic: [u8; 2],
// }
// fn main() {
//     env::set_var(
//         "WEBVIEW2_BROWSER_EXECUTABLE_FOLDER",
//         "./",
//     );
//     let mut pe = File::open("yan.exe").unwrap();
//     let mut dh = unsafe { mem::zeroed::<DosHeader>() };
//     let mz = pe.read_exact(& mut dh.magic);
//     // let mz=  std::ascii::escape_default(mz);

//     println!("{:X?}", dh.magic);
// }

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, io::Cursor, path::PathBuf};

use ico::*;
use rfd::FileDialog;
use wry::{
    self,
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Icon, Window, WindowBuilder},
    },
    http::Response,
    webview::{RpcRequest, RpcResponse, WebContext, WebViewBuilder},
    Value,
};
fn main() {
    // 使用指定版本的Webview2
    // 前提是要用expand 指令解压cab 文件
    // cab 文件需要在https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section 下载
    // env::set_var(
    //     "WEBVIEW2_BROWSER_EXECUTABLE_FOLDER",
    //     "D:\\code\\yan\\wv2",
    // );

    // 1. 创建窗口事件循环对象
    let evl = EventLoop::new();

    // use a window icon
    let by = include_bytes!("../logo128.ico");
    let icon_dir = IconDir::read(Cursor::new(by)).unwrap();
    let image = icon_dir.entries()[0].decode().unwrap();
    let rgba = image.rgba_data();
    let icon = Icon::from_rgba(rgba.to_vec(), image.height(), image.width()).unwrap();

    // 2. 创建窗口并绑定事件循环
    let win = WindowBuilder::new()
        .with_title("My App")
        .with_window_icon(Some(icon))
        .build(&evl)
        .unwrap();
    // 3. 在本地窗口种创建webview，并加载
    let web = WebViewBuilder::new(win).unwrap();

    // 调试模式下加载url
    #[cfg(debug_assertions)]
    let web = web.with_url("http://localhost:8341/").unwrap();

    // release 模式下加载html 文件
    // 需要自定义协议的支持
    #[cfg(not(debug_assertions))]
    let web = {
        use include_dir::{include_dir, Dir};
        use wry::http::{status::StatusCode, ResponseBuilder};
        const WEB_DIR: Dir = include_dir!("./web");
        web.with_custom_protocol("wry".into(), move |req| {
            // 1. 获取请求的url，
            // 如果协议格式为wry://index.html，程序会把index.html 做路径处理
            let path = req.uri().replace("wry://web/", "");

            // 2. 获取文件内容并设置响应类型
            let (code, data, meta) = match WEB_DIR.get_file(&path) {
                Some(file) => match path.split(".").last().unwrap() {
                    "html" => (200, file.contents(), mime::TEXT_HTML),
                    "js" => (200, file.contents(), mime::TEXT_JAVASCRIPT),
                    "css" => (200, file.contents(), mime::TEXT_CSS),
                    "ico" | "png" => (200, file.contents(), mime::IMAGE_PNG),
                    "jpg" => (200, file.contents(), mime::IMAGE_JPEG),
                    "bmp" => (200, file.contents(), mime::IMAGE_BMP),
                    _ => (404, "not found".as_bytes(), mime::TEXT_PLAIN),
                },
                None => (404, "not found".as_bytes(), mime::TEXT_PLAIN),
            };

            // 3. 构建返回体返回
            ResponseBuilder::new()
                .status(StatusCode::from_u16(code).unwrap())
                .mimetype(meta.essence_str())
                .body(Vec::from(data))
        })
        // 见步骤1.
        .with_url("wry://web/index.html")
        .unwrap()
    };

    let handler = move |window: &Window, mut req: RpcRequest| {
        let mut response = None;
        // 解析rpc 的参数
        println!("{:?}", req.params?.get(0).unwrap().get("message"));
        if &req.method == "fullscreen" {
        } else if &req.method == "send-parameters" {
            let file = FileDialog::new()
                .add_filter("text", &["txt", "rs"])
                .add_filter("rust", &["rs", "toml"])
                // .set_parent(window)  // 设置父窗口
                .pick_file()?;
            let path = file.to_path_buf().into_os_string().into_string().unwrap();
            response = Some(RpcResponse::new_result(
                req.id.take(),
                Some(Value::String(path)),
            ));
        }
        response
    };

    // 自定义user_data_directory 路径
    // 否则会在exe 下面出现一个文件夹，比较烦人
    // 还需要添加路径是否存在的判定
    let mut user_data_dir = PathBuf::from(env::var("appdata").unwrap());
    user_data_dir.push("yan");
    let mut webctx = WebContext::new(Some(user_data_dir));
    let mut web = web
        .with_rpc_handler(handler)
        .with_web_context(&mut webctx)
        .build()
        .unwrap();

    // 4. 设置事件处理
    evl.run(move |evt, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match evt {
            Event::NewEvents(StartCause::Init) => println!("app has started!"),
            Event::WindowEvent {
                window_id: _, // 忽略未使用变量警告
                event,
                .. // 匹配其他字段，虽然没有了
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(_) => {
                    let _ = web.resize();
                }
                _ => (),
            },
            _ => (),
        }
    });
}
