use reqwest::blocking::{Client, Response};
use random_string::generate;
use quick_protobuf::{MessageRead, BytesReader};

use crate::mangaplus_pb;

const BASE_URL: &str = "https://jumpg-webapi.tokyo-cdn.com";
const USER_AGENT: &str = "JumpPlus/1 CFNetwork/1333.0.4 Darwin/21.5.0";
const VERSION: &str = "47";
const OS_VERSION: &str = "10";
const OS_NAME: &str = "android";
const SECURITY_KEY_CONST: &str = "4Kin9vGg";

const HEX_CHARSET: &str = "0123456789abcdef";

pub struct MangaPlusClient {
    pub device_secret: String,
}

impl MangaPlusClient {
    pub fn new() -> Self {
        Self{
            device_secret: "".to_string(),
        }
    }

    fn _do_get(&self, url: String) -> Result<Response, String> {
        let res = Client::new()
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send();
        match res {
            Ok(response) => { return Ok(response) },
            Err(e) => { return Err(format!("error_http_request:{}", e)) },
        }
    }

    fn _do_put(&self, url: String, params: &[(&str, &str)]) -> Result<Response, String> {
        let res = Client::new()
            .put(url)
            .header("User-Agent", USER_AGENT)
            .form(params)
            .send();
        match res {
            Ok(response) => { return Ok(response) },
            Err(e) => { return Err(format!("error_http_request:{}", e)) },
        }
    }

    pub fn get_device_secret(&mut self) -> Result<String, String> {
        // generate a random device ID MD5
        // remember device_token = md5(device_id)
        // and security_key = md5(device_token + "4Kin9vGg")
        let device_token = generate(32, HEX_CHARSET);
        let security_key = format!("{:x}", md5::compute(format!("{}{}", device_token, SECURITY_KEY_CONST)));

        match self._do_put(format!("{}/api/register", BASE_URL), &[("device_token", &device_token), ("security_key", &security_key)]) {
            Err(e) => { return Err(e); },
            Ok(r) => {
                let body = r.text().unwrap();
                let mut reader = BytesReader::from_bytes(body.as_bytes());
                let response = mangaplus_pb::Response::from_reader(&mut reader, &body.as_bytes()).expect("cannot read Response ProtoBuf");
                //println!("{:?}", response);
                if let Some(result) = response.result {
                    self.device_secret = result.registration_data.unwrap().device_secret.to_string();
                    return Ok(self.device_secret.clone());
                } else if let Some(error) = response.error {
                    return Err(error.debug.unwrap().to_string());
                }
            }
        }

        Err("".to_string())
    }
}
