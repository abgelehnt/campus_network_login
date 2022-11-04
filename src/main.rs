use directories::ProjectDirs;
use reqwest::redirect;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{fs, thread, time::Duration};
use std::io;
use std::fs::File;
use std::ops::Add;
use log::{debug, info, LevelFilter, warn};

fn get_query_string(text: &String) -> Option<String> {
    if !text.contains("eportal/index.jsp?wlanuserip") {
        info!("Internet Connected.");
        return None;
    }

    let query_string = &text[69..text.len() - 12];
    Some(String::from(query_string))
}

fn post_result(data: &UserData) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(redirect::Policy::none())
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:106.0) Gecko/20100101 Firefox/106.0",
        )
        .http1_title_case_headers()
        .build()?;
    let res = client.get("http://baidu.com").send()?;
    let res = res.text()?;
    let res = get_query_string(&res);
    let res = match res {
        Some(s) => s,
        None => return Ok(true),
    };

    let params = [("userId", data.user_id.as_str()),
        ("password", data.password.as_str()),
        ("service", data.service.as_str()),
        ("queryString", res.as_str()),
        ("operatorPwd", ""),
        ("operatorUserId", ""),
        ("validcode", ""),
        ("passwordEncrypt", if data.password.len() > 100 { "true" } else { "false" })];

    let res = client
        .post(String::from("http://").add(data.ip.as_str()).add("/eportal/InterFace.do?method=login"))
        .form(&params)
        .send()?;

    let res_status = res.status();
    let res_text = res.text()?;
    debug!("Satus Code = {}\nBody = {}", res_status, res_text);
    let success = res_text.find("success").is_some();
    if success {
        info!("Internet Connect Successful.");
    }
    Ok(success)
}

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    user_id: String,
    password: String,
    ip: String,
    service: String,
}

fn get_user_data_from_stdin() -> UserData {
    let mut user_id = String::new();
    println!("请输入你的学号:");
    io::stdin().read_line(&mut user_id).unwrap();
    let user_id = String::from(user_id.trim());

    let mut password = String::new();
    println!("请输入你的密码:");
    io::stdin().read_line(&mut password).unwrap();
    let password = String::from(password.trim());

    let mut ip = String::new();
    println!("请输入你的ip（默认为扬大校园网：10.240.0.97）:");
    io::stdin().read_line(&mut ip).unwrap();
    if ip.trim().len() > 5 {
        ip = String::from(ip.trim());
    } else {
        ip = String::from("10.240.0.97");
    }

    let mut service = String::new();
    println!("服务1：学校互联网服务，服务2：联通互联网服务，服务3：移动互联网服务，服务4：电信互联网服务，服务5：校内免费服务\n请输入你的服务:");
    io::stdin().read_line(&mut service).unwrap();
    let service_num = String::from(service.trim()).parse::<i32>().unwrap();
    match service_num {
        1 => service = String::from("%E5%AD%A6%E6%A0%A1%E4%BA%92%E8%81%94%E7%BD%91%E6%9C%8D%E5%8A%A1"),
        2 => service = String::from("%E8%81%94%E9%80%9A%E4%BA%92%E8%81%94%E7%BD%91%E6%9C%8D%E5%8A%A1"),
        3 => service = String::from("%E7%A7%BB%E5%8A%A8%E4%BA%92%E8%81%94%E7%BD%91%E6%9C%8D%E5%8A%A1"),
        4 => service = String::from("%E7%94%B5%E4%BF%A1%E4%BA%92%E8%81%94%E7%BD%91%E6%9C%8D%E5%8A%A1"),
        5 => service = String::from("%E6%A0%A1%E5%86%85%E5%85%8D%E8%B4%B9%E6%9C%8D%E5%8A%A1"),
        _ => {
            println!("错误");
            exit(0)
        }
    }

    thread::sleep(Duration::from_micros(500));

    UserData {
        user_id,
        password,
        ip,
        service,
    }
}

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "Chi", "CampusNetworkLogin").unwrap();
    let config_dir = proj_dirs.config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap();
    }
    config_dir.join("config.json")
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<UserData, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    let mut edit_data = false;
    let mut logger = false;
    for argument in std::env::args() {
        if argument == "-e" {
            edit_data = true;
        }else if argument == "-v" {
            logger = true;
        }
    }

    env_logger::builder().filter_level(if logger { LevelFilter::Debug } else { LevelFilter::Off }).init();

    let read_result = read_json_from_file(get_config_path());
    let data;
    if read_result.is_ok() & !edit_data {
        data = read_result.unwrap();
    } else {
        data = get_user_data_from_stdin();
        fs::write(get_config_path(), serde_json::to_string(&data).unwrap()).unwrap();
    }

    for i in 0..30 {
        if post_result(&data).unwrap_or_else(|e| {
            info!("{}: {}", i, e);
            false
        }) {
            exit(0);
        }
        thread::sleep(Duration::from_micros(500));
    }
    warn!("Connect Error");
    exit(1);
}
