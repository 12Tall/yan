// mod http_server;
// use http_server::start_server;
// use std::thread;

// /// # DllMain  
// /// > dll 的入口函数
// #[no_mangle]
// pub extern "stdcall" fn DllMain(
//     _hinst_dll: winapi::shared::minwindef::HINSTANCE,
//     fdw_reason: u32,
//     _: *mut winapi::ctypes::c_void,
// ) {
//     if fdw_reason == 1 {
//         // thread::spawn(start_server);
//     }
// }
