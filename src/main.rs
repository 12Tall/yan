// release 版，关闭控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
use std::io::Result;

use rfd::FileDialog;
use utils::{check_process_mutex, read_icon};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::ControlFlow,
        window::Window,
    },
    webview::{RpcRequest, RpcResponse},
    Value,
};

use crate::utils::{create_app, create_webview};

fn main() -> Result<()> {
    let _inst = check_process_mutex()?;
    let icon = read_icon(include_bytes!("../res/logo128.ico"));
    let (win, evl) = create_app(icon);
    let webview = create_webview(win, move |_window: &Window, mut req: RpcRequest| {
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
    });

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
                    let _ = webview.resize();
                }
                _ => (),
            },
            _ => (),
        }
    });

    // Ok(())
}

