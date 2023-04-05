mod portal;
use std::{collections::HashMap};

use scraper::{Html, Selector};
use reqwest::{Client};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let auth_data = portal::AuthData::new(
        "00M00000",
        "PASSWORD",
        [
            ['X','X','X','X','X','X','X','X','X','X'],
            ['X','X','X','X','X','X','X','X','X','X'],
            ['X','X','X','X','X','X','X','X','X','X'],
            ['X','X','X','X','X','X','X','X','X','X'],
            ['X','X','X','X','X','X','X','X','X','X'],
            ['X','X','X','X','X','X','X','X','X','X'],
            ['X','X','X','X','X','X','X','X','X','X']
        ]
    );
    let client = Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    // password form を取得
    let password_form_url = "https://portal.nap.gsic.titech.ac.jp/GetAccess/Login?Template=userpass_key&AUTHMETHOD=UserPassword";
    let password_form_req = client.get(password_form_url)
        .send().await?;
    let mut j_session_id = String::new();
    let cookies = password_form_req.cookies();
    println!("Cookies");
    for cookie in cookies{
        println!("{} : {}", cookie.name(),cookie.value());
        if cookie.name() == "JSESSIONID"{
            j_session_id = format!("JSESSIONID={}",cookie.value());
        }
    }
    let body = password_form_req.text().await?;

    // password form をパース
    let document = Html::parse_document(&body);
    let selector = Selector::parse("input").unwrap();
    let mut password_form = HashMap::new();
    for title in document.select(&selector){
        let name = title.value().attr("name").unwrap();
        if title.value().attr("value").is_none() {
            let value = "";
            password_form.insert(name, value);
        }else{
            let value = title.value().attr("value").unwrap();
            password_form.insert(name, value);
        }
    }

    // 認証情報を追加
    password_form.insert("usr_name", &auth_data.student_id);
    password_form.insert("usr_password", &auth_data.password);

    // println!("----------");
    // println!("send password_form");
    // for e in &password_form{
    //     print!("{}={}&", e.0,e.1)
    // }
    // println!("----------");

    // password form を送信、matrix form を取得
    let req = client.post("https://portal.nap.gsic.titech.ac.jp/GetAccess/Login")
        .form(&password_form)
        .header(reqwest::header::REFERER, password_form_url)
        .header(reqwest::header::COOKIE, j_session_id)
        .send().await?;
    let cookies = req.cookies();
    println!("Cookies");
    for cookie in cookies{
        println!("{} : {}", cookie.name(),cookie.value());
    }
    let body = req.text().await?;

    // matrix form をパース
    let document = Html::parse_document(&body);
    let selector = Selector::parse("input").unwrap();
    let mut matrix_form = HashMap::new();
    for title in document.select(&selector){
        let name = title.value().attr("name").unwrap();
        if title.value().attr("value").is_none() {
            let value = "".to_string();
            matrix_form.insert(name, value);
        }else{
            let value = title.value().attr("value").unwrap().to_string();
            matrix_form.insert(name, value);
        }
    }

    // matrix form を記入
    let selector = Selector::parse("tr").unwrap();
    let selector_matrix = Selector::parse("th[align=\"left\"]").unwrap();
    let selector_input = Selector::parse("input").unwrap();

    for nodes in document.select(&selector){
        let mut matrix = "".to_string();
        let matrix_node = nodes.select(&selector_matrix).next();
        if !matrix_node.is_none() {
            matrix = matrix_node.unwrap().inner_html();
        }

        if !matrix.is_empty(){
            let row: char = matrix.chars().nth(1).unwrap();
            let line: u8 = matrix.chars().nth(3).unwrap() as u8 - 48;
            let code = auth_data.pick_code(row, line).clone();

            matrix_form.insert(
                nodes.select(&selector_input)
                    .next().unwrap()
                    .value().attr("name").unwrap(),
                code
            );
        }
    }

    // println!("----------");
    // println!("matrix_form");
    // for m in &matrix_form{
    //     println!("{} : {}",m.0,m.1);
    // }
    // println!("----------");

    // matrix form を送信、portal sessionを取得
    let req = client.post("https://portal.nap.gsic.titech.ac.jp/GetAccess/Login")
        .form(&matrix_form)
        .header(reqwest::header::REFERER, "https://portal.nap.gsic.titech.ac.jp/GetAccess/Login?Template=idg_key&AUTHMETHOD=IG&GASF=CERTIFICATE,IG.GRID,IG.TOKENRO,IG.OTP&LOCALE=ja_JP&GAREASONCODE=13&GAIDENTIFICATIONID=UserPassword&GARESOURCEID=resourcelistID2&GAURI=https://portal.nap.gsic.titech.ac.jp/GetAccess/ResourceList&Reason=13&APPID=resourcelistID2&URI=https://portal.nap.gsic.titech.ac.jp/GetAccess/ResourceList")
        .send().await?;

    // Moodle session を取得
    let req = client.get("https://t2schola.titech.ac.jp:443/auth/eltitech/autologin.php")
        .send().await?;
    let cookies = req.cookies();
    println!("Cookies moodle");
    for cookie in cookies {
        println!("{},{}",cookie.name(),cookie.value());
    }

    // Moodle web service token を取得
    let req = client.get("https://t2schola.titech.ac.jp/admin/tool/mobile/launch.php?service=moodle_mobile_app&passport=14029&urlscheme=mary")
        .send().await?;

    Ok(())
}

