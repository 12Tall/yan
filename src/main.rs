#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use wry::{
    self,
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

fn main() {
    // #[cfg(not(debug_assertions))]
    // 1. 创建窗口事件循环对象
    let evl = EventLoop::new();
    // 2. 创建窗口并绑定事件循环
    let win = WindowBuilder::new()
        .with_title("My App")
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
        use wry::{http::{status::StatusCode, ResponseBuilder}};
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

    let web = web.build().unwrap();

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
