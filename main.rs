use std::env;
use std::fs;
use tiny_http::{Response, Server};

#[derive(serde::Deserialize)]
struct SaveRequest {
    content: String,
}

static mut MD_FILE_PATH: Option<String> = None;

fn read_file_content() -> String {
    let path = unsafe { MD_FILE_PATH.as_ref().expect("MD_FILE_PATH not set") };
    fs::read_to_string(path).unwrap_or_default()
}

fn main() {
    //1,参数获取
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("参数错误");
        std::process::exit(1);
    }
    let md_path = args[1].clone();
    unsafe {
        MD_FILE_PATH = Some(md_path.clone());
    }
    //2,启动服务器
    let server = Server::http("127.0.0.1:8080").expect("Failed to bind");
    println!("✅ Open http://127.0.0.1:8080");
    println!("📄 Editing: {}", md_path);

    for mut request in server.incoming_requests() {
        match (request.method().as_str(), request.url()) {
            //主页
            ("GET", "/") => {
                let current_content = read_file_content();
                let escaped = current_content
                    .replace("\\", "\\\\")
                    .replace("\r", "\\r")
                    .replace("\n", "\\n")
                    .replace("\"", "\\\"");
                let html = include_str!("../static/index.html");
                let response_body = html.replace("{{INITIAL_MD}}", &escaped);
                let response = Response::from_string(response_body).with_header(
                    tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"text/html; charset=utf-8"[..],
                    )
                    .unwrap(),
                );
                let _ = request.respond(response);
            }
            //获取文本内容
            ("GET", "/content") => {
                let content = read_file_content();
                let response = Response::from_string(content).with_header(
                    tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"text/plain; charset=utf-8"[..],
                    )
                    .unwrap(),
                );
                let _ = request.respond(response);
            }
            //更新文本内容
            ("POST", "/save") => {
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);

                match serde_json::from_str::<SaveRequest>(&body) {
                    Ok(req) => {
                        let path = unsafe { MD_FILE_PATH.as_ref().unwrap() };
                        if let Err(e) = fs::write(path, &req.content) {
                            eprintln!("Save error: {}", e);
                            let _ = request.respond(
                                Response::from_string("Save failed").with_status_code(500),
                            );
                        } else {
                            println!("💾 Saved to {}", path);
                            let _ = request.respond(Response::from_string("OK"));
                        }
                    }
                    Err(e) => {
                        eprintln!("JSON error: {}", e);
                        let _ = request
                            .respond(Response::from_string("Invalid JSON").with_status_code(400));
                    }
                }
            }
            _ => {
                let _ = request.respond(Response::from_string("404").with_status_code(404));
            }
        }
    }
}
