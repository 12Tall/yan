use std::path::{Path, PathBuf};

use rocket::{
    self,
    fs::{relative, NamedFile},
    get,
    response::Redirect,
    routes,
};

#[get("/")]
async fn index() -> Redirect {
    // 重定向至index.html
    Redirect::permanent("/index.html")
}
/// 处理静态资源
#[get("/<file..>")]
async fn assets(file: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(relative!("web")).join(file);
    if path.is_dir() {
        path.push("index.html");
    }
    NamedFile::open(path).await.ok()
}

pub fn start_web_server() {
    rocket::async_main(async move {
        let cli = rocket::build()
            .mount("/", routes![index, assets])
            .launch()
            .await;
        cli.unwrap();
    })
}
