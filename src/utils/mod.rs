use ico::IconDir;
use rfd::MessageDialog;
use single_instance::SingleInstance;
use std::{
    env,
    io::{Cursor, Error, ErrorKind},
    path::PathBuf,
};
use wry::{
    application::{
        event_loop::{ EventLoop, },
        window::{Icon, Window, WindowBuilder},
    },
    webview::{RpcRequest, RpcResponse, WebContext, WebView, WebViewBuilder},
};

pub fn check_process_mutex() -> Result<SingleInstance, Error> {
    let inst = SingleInstance::new("yan").unwrap();

    match inst.is_single() {
        true => Ok(inst),
        _ => {
            MessageDialog::new()
                .set_title("警告")
                .set_description("该程序已经运行！")
                .show();
            Err(Error::new(ErrorKind::AlreadyExists, "进程实例已存在！"))
        }
    }
}

pub fn read_icon(icon_binary: &[u8]) -> Icon {
    let icon_dir = IconDir::read(Cursor::new(icon_binary)).unwrap();
    let image = icon_dir.entries()[0].decode().unwrap();
    let rgba = image.rgba_data();
    Icon::from_rgba(rgba.to_vec(), image.height(), image.width()).unwrap()
}

pub fn create_app(icon: Icon) -> (Window, EventLoop<()>) {
    let evl = EventLoop::new();

    let win = WindowBuilder::new()
        .with_title("My App")
        .with_window_icon(Some(icon))
        .build(&evl)
        .unwrap();

    (win, evl)
}

pub fn create_webview<F>(win: Window, rpc_handler: F) -> WebView
where
    F: Fn(&Window, RpcRequest) -> Option<RpcResponse> + 'static,
{
    let web = WebViewBuilder::new(win).unwrap();

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
            ResponseBuilder::new()
                .status(StatusCode::from_u16(code).unwrap())
                .mimetype(meta.essence_str())
                .body(Vec::from(data))
        })
        .with_url("wry://web/index.html")
        .unwrap()
    };

    #[cfg(debug_assertions)]
    let web = web.with_url("http://localhost:8341/").unwrap();

    let mut user_data_dir = PathBuf::from(env::var("appdata").unwrap());
    user_data_dir.push("yan");
    let mut webctx = WebContext::new(Some(user_data_dir));
    let web = web
        .with_rpc_handler(rpc_handler)
        .with_web_context(&mut webctx)
        .build()
        .unwrap();
    web
}
