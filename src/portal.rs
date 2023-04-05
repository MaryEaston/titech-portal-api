use std::ptr::null;

use reqwest::{Client, cookie::Cookie};
use reqwest::{Response};

pub struct Portal{
    auth_session_id : String,
    response : Response,
    auth_data : AuthData,
}

impl Portal{
    // pub fn set_student_id(&self, id: String) -> Portal {
    //     self.student_id = id;
    //     return *self;
    // }
    // pub fn set_password(&self, password: String) -> Portal {
    //     self.password = password;
    //     return *self;
    // }
    // fn new(auth_data: AuthData) -> Self {
    //     Portal {
    //         auth_session_id: String::from(""),
    //         response: Response::new(),
    //         auth_data: auth_data,
    //     }
    // }

    // async fn auth(&self){

    // }
}

pub struct AuthData{
    pub student_id : String,
    pub password : String,
    pub matrix_code: [[char; 10]; 7]
}

impl AuthData {
    // pub fn set_matrix_code(&self, matrix_code: [[char; 10]; 7]) -> MatrixCode {
    //     self.matrix_code = matrix_code;
    //     return *self;
    // }
    pub fn new(studet_id: &str, password: &str, matrix_code: [[char; 10]; 7]) -> Self{
        AuthData {
            student_id: String::from(studet_id),
            password: String::from(password),
            matrix_code: matrix_code
        }
    }

    pub fn pick_code(&self, row:char, line:u8) -> String{
        let row_index: u8 = row as u8 - 'A' as u8;
        let line_index: u8 = line - 1;
        return self.matrix_code[line_index as usize][row_index as usize].to_string();
    }
}

// curl -X POST 'https://portal.nap.gsic.titech.ac.jp/GetAccess/Login' \
// -H 'Referer:Referer: https://portal.nap.gsic.titech.ac.jp/portal.pl?GASF=CERTIFICATE,IG.GRID,IG.TOKENRO,IG.OTP&GAREASONCODE=-1&GARESOURCEID=T2SCHOLA2023_S001_80&GAURI=https://t2schola.titech.ac.jp/auth/eltitech/autologin.php&Reason=-1&APPID=T2SCHOLA2023_S001_80&URI=https://t2schola.titech.ac.jp/auth/eltitech/autologin.php' \
// -d 'usr_name=23M12522' \
// -d 'AUTHMETHOD=' \
// -d 'usr_password=Da1104111' \
// -d 'pageGenTime=1680750198071' \
// -d 'GARESOURCEID=secureocwstudent2019' \
// -d 'AUTHTYPE=' \
// -d 'CSRFFormToken=bc01754c6a06c772f1b4c8cdfced93803169a4557a30dbd1' \
// -d 'LOCALE=ja_JP' \
// -d 'HiddenURI=https://secure.ocw.titech.ac.jp/ocwi/index.php%3Fmodule%3DOcwi%26action%3DLogin' \
// -d 'Template=userpass_key' \
// -d 'OK="    OK    "' \
// -c cookie.txt
