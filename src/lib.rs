use reqwest::header::AUTHORIZATION;
use reqwest::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CodeNameResp {
    name: String,
    #[serde(rename = "idNo")]
    id_no: String,
    #[serde(rename = "respMessage")]
    resp_message: String,
    #[serde(rename = "respCode")]
    resp_code: String,
    province: Option<String>,
    city: Option<String>,
    county: Option<String>,
    birthday: Option<String>,
    sex: Option<String>,
    age: Option<String>,
}

fn validate_name(idcard: &str, name: &str, appcode: &str) -> Result<CodeNameResp, Error> {
    let url = "https://idenauthen.market.alicloudapi.com/idenAuthentication";
    let params = [("idNo", idcard), ("name", name)];
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .form(&params)
        .header(AUTHORIZATION, format!("APPCODE {}", appcode))
        .send()?
        .json::<CodeNameResp>()?;

    Ok(res)
}

fn validate_idcard(idcard: &str) -> bool {
    let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let sum: u32 = idcard
        .chars()
        .take(17)
        .zip(weights.iter())
        .map(|(d, w)| d.to_digit(10).unwrap_or(10) * w)
        .sum();

    let code = match sum % 11 {
        0 => '1',
        1 => '0',
        2 => 'X',
        3 => '9',
        4 => '8',
        5 => '7',
        6 => '6',
        7 => '5',
        8 => '4',
        9 => '3',
        10 => '2',
        _ => ' ',
    };

    match idcard.chars().last() {
        Some(v) => code == v.to_ascii_uppercase(),
        None => false,
    }
}

pub fn validate(idcard: &str, name: Option<&str>, appcode: Option<&str>) -> Result<bool, Error> {
    match validate_idcard(idcard) {
        true => match name {
            Some(v) => match validate_name(idcard, v, appcode.unwrap_or("")) {
                Ok(x) => Ok(x.resp_code == "0000"),
                Err(ex) => Err(ex),
            },

            None => Ok(true),
        },

        false => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::validate("510108197205052138", None, None).unwrap(),
            false
        );

        assert_eq!(
            super::validate(
                "510108197205052137",
                Some("无名氏"),
                Some("e61152457c5d41f99d383868d97e328e")
            )
            .unwrap(),
            false
        );

        assert_eq!(
            super::validate(
                "510108197205052137",
                Some("苏渝"),
                Some("e61152457c5d41f99d383868d97e328e")
            )
            .unwrap(),
            true
        );

        assert_eq!(super::validate_idcard("510108197205052137"), true);
        assert_eq!(super::validate_idcard("15040419840217262X"), true);
        assert_eq!(super::validate_idcard("15040419840217262x"), true);
        assert_eq!(super::validate_idcard("150404198402172620"), false);
        assert_eq!(super::validate_idcard("150404198402"), false);
        assert_eq!(super::validate_idcard("1"), false);
        assert_eq!(super::validate_idcard(""), false);

        assert_eq!(
            super::validate_name(
                "510108197205052137",
                "苏渝",
                "e61152457c5d41f99d383868d97e328e"
            )
            .unwrap()
            .resp_code,
            "0000"
        );
        assert_eq!(
            super::validate_name(
                "510108197205052138",
                "苏渝",
                "e61152457c5d41f99d383868d97e328e"
            )
            .unwrap()
            .resp_code,
            "0004"
        );
    }
}
