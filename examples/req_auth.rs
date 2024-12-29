use clap::Parser;
use colored::*;
use reqwest::redirect::Policy;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 用户名
    #[arg(short, long)]
    username: String,

    /// 密码
    #[arg(short, long)]
    password: String,

    /// 是否从外部 JSON 读取课程信息，默认是（也只做了是
    #[arg(short, long, default_value_t = true)]
    json: bool,

    /// 存放json 的文件夹地址，默认 all_courses
    #[arg(short, long)]
    folder_addr: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // parse params
    let args = Args::parse();

    let path = args.folder_addr.unwrap_or("./all_courses".to_string());
    let courses_dir = Path::new(&path);

    if !courses_dir.is_dir() {
        eprintln!("The directory {} does not exist.", courses_dir.display());
    }

    // new client, with cookie store
    let client = Client::builder()
        .redirect(Policy::limited(5))
        .cookie_store(true)
        .build()
        .unwrap();

    // authenticate
    println!("{}", "Authenticating...".blue().bold());
    let response = client
        .get("https://ids.hit.edu.cn/authserver/combinedLogin.do?type=IDSUnion&appId=ff2dfca3a2a2448e9026a8c6e38fa52b&success=http%3A%2F%2Fjw.hitsz.edu.cn%2FcasLogin")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);
    let input_selector = Selector::parse("form#authZForm input[type=hidden]").unwrap();

    let mut form_data = HashMap::new();
    form_data.insert("username", args.username);
    form_data.insert("password", args.password);

    for input in document.select(&input_selector) {
        if let Some(name) = input.value().attr("name") {
            if let Some(value) = input.value().attr("value") {
                form_data.insert(name, value.to_string());
            }
        }
    }

    let _response = client
        .post("https://sso.hitsz.edu.cn:7002/cas/oauth2.0/authorize")
        .form(&form_data)
        .send()
        .await?;

    println!("{}", "Login success.".green().bold());
    // auth end.

    if args.json {
        for entry in fs::read_dir(courses_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file = File::open(&path).expect("unable to open file.");
                let to_choose: Value = serde_json::from_reader(file)?;

                list_all_course(to_choose["kxrwList"]["list"].as_array().unwrap_or(&vec![]));
            }
        }

        for entry in fs::read_dir(courses_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file = File::open(&path).expect("unable to open file.");
                let to_choose: Value = serde_json::from_reader(file)?;

                let _res = choose_course(
                    to_choose["kxrwList"]["list"].as_array().unwrap_or(&vec![]),
                    client.clone(),
                    to_choose["xsxkPage"]["p_xkfsdm"]
                        .as_str()
                        .expect("no method specified."),
                    // "xx-b-b",
                    to_choose["xsxkPage"]["p_dqxnxq"]
                        .as_str()
                        .expect("no ...dqxnxq specified."),
                    to_choose["xsxkPage"]["p_xn"]
                        .as_str()
                        .expect("no year specified."),
                    to_choose["xsxkPage"]["p_xnxq"]
                        .as_str()
                        .expect("no year_semister specified."),
                    to_choose["xsxkPage"]["p_xq"]
                        .as_str()
                        .expect("no semister specified."),
                    to_choose["xsxkPage"]["p_dqxn"]
                        .as_str()
                        .expect("no semister specified."),
                    to_choose["xsxkPage"]["p_dqxq"]
                        .as_str()
                        .expect("no semister specified."),
                    to_choose["xsxkPage"]["p_sfgldjr"]
                        .as_str()
                        .expect("no semister specified."),
                    to_choose["xsxkPage"]["p_pylx"]
                        .as_str()
                        .expect("no semister specified."),
                )
                .await?;
            }
        }

        return Ok(());
    }

    // get courses.
    // let mut body = init_req_body();

    // let response = client
    //     .post("http://jw.hitsz.edu.cn/Xsxk/queryXkdqXnxq")
    //     .header("Connection", "keep-alive")
    //     .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
    //     .body("cxsfmt=0&p_pylx=1&mxpylx=1&p_sfgldjr=0&p_sfredis=0&p_sfsyxkgwc=0&p_xktjz=&p_chaxunxh=&p_gjz=&p_skjs=&p_xn=&p_xq=&p_xnxq=&p_dqxn=&p_dqxq=&p_dqxnxq=&p_xkfsdm=&p_xiaoqu=&p_kkyx=&p_kclb=&p_xkxs=&p_dyc=&p_kkxnxq=&p_id=&p_sfhlctkc=0&p_sfhllrlkc=0&p_kxsj_xqj=&p_kxsj_ksjc=&p_kxsj_jsjc=&p_kcdm_js=&p_kcdm_cxrw=&p_kc_gjz=&p_xzcxtjz_nj=&p_xzcxtjz_yx=&p_xzcxtjz_zy=&p_xzcxtjz_zyfx=&p_xzcxtjz_bj=&p_sfxsgwckb=1&p_skyy=")
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // let params = from_str::<Value>(&response)?;
    // if let Value::Object(map) = params {
    //     for (key, value) in map.iter() {
    //         body.insert(key.to_string(), value.as_str().unwrap().to_string());
    //     }
    // }

    // let response = client
    //     .post("http://jw.hitsz.edu.cn/Xsxk/queryKxrw")
    //     .header("Connection", "keep-alive")
    //     .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
    //     .body("cxsfmt=0&p_pylx=1&mxpylx=1&p_sfgldjr=0&p_sfredis=0&p_sfsyxkgwc=0&p_xktjz=&p_chaxunxh=&p_gjz=&p_skjs=&p_xn=2024-2025&p_xq=2&p_xnxq=2024-20252&p_dqxn=2024-2025&p_dqxq=1&p_dqxnxq=2024-20251&p_xkfsdm=bx-b-b&p_xiaoqu=&p_kkyx=&p_kclb=&p_xkxs=&p_dyc=&p_kkxnxq=&p_id=&p_sfhlctkc=0&p_sfhllrlkc=0&p_kxsj_xqj=&p_kxsj_ksjc=&p_kxsj_jsjc=&p_kcdm_js=&p_kcdm_cxrw=&p_kc_gjz=&p_xzcxtjz_nj=&p_xzcxtjz_yx=&p_xzcxtjz_zy=&p_xzcxtjz_zyfx=&p_xzcxtjz_bj=&p_sfxsgwckb=1&p_skyy=&p_chaxunxkfsdm=&pageNum=1&pageSize=100")
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // let must_course = from_str::<Value>(&response)?;
    // let binding = vec![];
    // let must_course = must_course["kxrwList"]["list"]
    //     .as_array()
    //     .unwrap_or(&binding);

    // let response = client
    //     .post("http://jw.hitsz.edu.cn/Xsxk/queryKxrw")
    //     .header("Connection", "keep-alive")
    //     .header(
    //         "Content-Type",
    //         "application/x-www-form-urlencoded; charset=UTF-8",
    //     )
    //     // .form(&body)
    //     .body("cxsfmt=0&p_pylx=1&mxpylx=1&p_sfgldjr=0&p_sfredis=0&p_sfsyxkgwc=0&p_xktjz=&p_chaxunxh=&p_gjz=&p_skjs=&p_xn=2024-2025&p_xq=2&p_xnxq=2024-20252&p_dqxn=2024-2025&p_dqxq=1&p_dqxnxq=2024-20251&p_xkfsdm=ty-b-b&p_xiaoqu=&p_kkyx=&p_kclb=&p_xkxs=&p_dyc=&p_kkxnxq=&p_id=&p_sfhlctkc=0&p_sfhllrlkc=0&p_kxsj_xqj=&p_kxsj_ksjc=&p_kxsj_jsjc=&p_kcdm_js=&p_kcdm_cxrw=&p_kc_gjz=&p_xzcxtjz_nj=&p_xzcxtjz_yx=&p_xzcxtjz_zy=&p_xzcxtjz_zyfx=&p_xzcxtjz_bj=&p_sfxsgwckb=1&p_skyy=&p_chaxunxkfsdm=&pageNum=1&pageSize=100")
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // let pe_course = from_str::<Value>(&response)?;
    // let pe_course = pe_course["kxrwList"]["list"].as_array().unwrap_or(&binding);

    // println!("{}", "All courses:".green().bold());
    // list_all_course(must_course);
    // list_all_course(pe_course);

    // println!("{}", "Choose courses:".green().bold());
    // let _res = choose_course(must_course, client.clone()).await?;
    // let _res = choose_course(pe_course, client).await?;

    Ok(())
}

async fn choose_course(
    all_course: &Vec<Value>,
    client: Client,
    choose_method: &str,
    dq_year_semister: &str,
    year: &str,
    year_semister: &str,
    semister: &str,
    dq_year: &str,
    dq_semister: &str,
    sfgldjr:&str,
    pylx:&str
) -> Result<(), Box<dyn Error>> {
    for course in all_course {
        println!(
            "\n\nCourse id: {}, Course name: {}, Course teacher: {} {}",
            course["id"].as_str().unwrap().green().bold(),
            course["kcmc"].as_str().unwrap().purple().bold(),
            course["dgjsmc"].as_str().unwrap_or("").blue().bold(),
            course["tyxmmc"].as_str().unwrap_or("").blue().bold(),
        );

        println!("{}", "Choose this? Yes: Y/y/yes, else no.".green().bold());
        let mut buffer = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here
        stdin.read_line(&mut buffer)?;
        let yes = buffer.trim().to_lowercase() == "y" || buffer.trim().to_lowercase() == "yes";

        if yes {
            // println!(
            //     "{}\n{}\n",
            //     "request curl command:".green().bold(),
            //     curl_request_str(cookie, course["id"].as_str().unwrap().green().bold())
            // );

            let res = curl_request(
                client.clone(),
                course["id"].as_str().unwrap(),
                choose_method,
                dq_year_semister,
                year,
                year_semister,
                semister,
                dq_year,
                dq_semister,
                sfgldjr,
                pylx
            )
            .await;
            let _ = match res {
                Ok(res) => println!("{} {}", "Success:".green().bold(), res),
                Err(e) => println!("{}", e.to_string().red().bold()),
            };
        } else {
            println!("{}", "Skip this course.".red().bold());
            continue;
        }
    }

    Ok(())
}

fn list_all_course(all_course: &Vec<Value>) {
    for course in all_course {
        println!(
            "Course id: {}, Course name: {}, Course teacher: {} {}",
            course["id"].as_str().unwrap().green().bold(),
            course["kcmc"].as_str().unwrap().purple().bold(),
            course["dgjsmc"].as_str().unwrap_or("").blue().bold(),
            course["tyxmmc"].as_str().unwrap_or("")
        )
    }
}

async fn curl_request(
    client: Client,
    id: &str,
    choose_method: &str,
    dq_year_semister: &str,
    year: &str,
    year_semister: &str,
    semister: &str,
    dq_year: &str,
    dq_semister: &str,
    sfgldjr: &str,
    pylx:&str
) -> Result<String, Box<dyn Error>> {
    let response = client.post("http://jw.hitsz.edu.cn/Xsxk/addGouwuche")
        .header("Connection", "keep-alive")
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .body(format!("cxsfmt=0&p_pylx={}&mxpylx=1&p_sfgldjr={}&p_sfredis=0&p_sfsyxkgwc=0&p_xktjz=rwtjzyx&p_chaxunxh=&p_gjz=&p_skjs=&p_xn={}&p_xq={}&p_xnxq={}&p_dqxn={}&p_dqxq={}&p_dqxnxq={}&p_xkfsdm={}&p_xiaoqu=&p_kkyx=&p_kclb=&p_xkxs=&p_dyc=&p_kkxnxq=&p_id={}&p_sfhlctkc=0&p_sfhllrlkc=0&p_kxsj_xqj=&p_kxsj_ksjc=&p_kxsj_jsjc=&p_kcdm_js=&p_kcdm_cxrw=&p_kc_gjz=&p_xzcxtjz_nj=&p_xzcxtjz_yx=&p_xzcxtjz_zy=&p_xzcxtjz_zyfx=&p_xzcxtjz_bj=&p_sfxsgwckb=1&p_skyy=&p_chaxunxkfsdm=&pageNum=1&pageSize=18",
            pylx,
            sfgldjr,
            year,
            semister,
            year_semister,
            dq_year,
            dq_semister,
            dq_year_semister,
            choose_method, 
            id))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

// #[allow(unused)]
// fn init_req_body() -> HashMap<String, String> {
//     let mut body = HashMap::new();
//     body.insert("cxsfmt".to_string(), "0".to_string());
//     body.insert("p_pylx".to_string(), "1".to_string());
//     body.insert("mxpylx".to_string(), "1".to_string());
//     body.insert("p_sfgldjr".to_string(), "0".to_string());
//     body.insert("p_sfredis".to_string(), "0".to_string());
//     body.insert("p_sfsyxkgwc".to_string(), "0".to_string());
//     body.insert("p_xktjz".to_string(), "".to_string());
//     body.insert("p_chaxunxh".to_string(), "".to_string());
//     body.insert("p_gjz".to_string(), "".to_string());
//     body.insert("p_skjs".to_string(), "".to_string());
//     body.insert("p_xn".to_string(), "2024-2025".to_string());
//     body.insert("p_xq".to_string(), "2".to_string());
//     body.insert("p_xnxq".to_string(), "2024-20252".to_string());
//     body.insert("p_dqxn".to_string(), "2024-2025".to_string());
//     body.insert("p_dqxq".to_string(), "1".to_string());
//     body.insert("p_dqxnxq".to_string(), "2024-20251".to_string());
//     body.insert("p_xkfsdm".to_string(), "bx-b-b".to_string());
//     body.insert("p_xiaoqu".to_string(), "".to_string());
//     body.insert("p_kkyx".to_string(), "".to_string());
//     body.insert("p_kclb".to_string(), "".to_string());
//     body.insert("p_xkxs".to_string(), "".to_string());
//     body.insert("p_dyc".to_string(), "".to_string());
//     body.insert("p_kkxnxq".to_string(), "".to_string());
//     body.insert("p_id".to_string(), "".to_string());
//     body.insert("sfhlctkc".to_string(), "0".to_string());
//     body.insert("sfhllrlkc".to_string(), "0".to_string());
//     body.insert("p_kxsj_xqj".to_string(), "".to_string());
//     body.insert("p_kxsj_ksjc".to_string(), "".to_string());
//     body.insert("p_kxsj_jsjc".to_string(), "".to_string());
//     body.insert("p_kcdm_js".to_string(), "".to_string());
//     body.insert("p_kcdm_cxrw".to_string(), "".to_string());
//     body.insert("p_kc_gjz".to_string(), "".to_string());
//     body.insert("p_xzcxtjz_nj".to_string(), "".to_string());
//     body.insert("p_xzcxtjz_yx".to_string(), "".to_string());
//     body.insert("p_xzcxtjz_zy".to_string(), "".to_string());
//     body.insert("p_xzcxtjz_zyfx".to_string(), "".to_string());
//     body.insert("sfxsgwckb".to_string(), "1".to_string());
//     body.insert("skyy".to_string(), "".to_string());
//     body.insert("chaxunxkfsdm".to_string(), "".to_string());
//     body.insert("pageNum".to_string(), "1".to_string());
//     body.insert("pageSize".to_string(), "17".to_string());

//     body
// }
