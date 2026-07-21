use crate::error::Error;

pub struct Credentials {
    account_type: String,
    open_id: String,
    ams_token: String,
    ams_token_time: String,
    ams_session_token: String,
}

impl Credentials {
    pub fn from_cookies(cookies_string: &str) -> Result<Self, Error> {
        let mut account_type: Option<&str> = None;
        let mut open_id: Option<&str> = None;
        let mut ams_token: Option<&str> = None;
        let mut ams_token_time: Option<&str> = None;
        let mut ams_session_token: Option<&str> = None;

        for pair in cookies_string.split(";") {
            if let Some(equal_sign_index) = pair.find("=") {
                let (key, value) = pair.split_at(equal_sign_index);
                let key = key.trim();
                let value = value[1..].trim(); // 跳过等于号本身
                match key {
                    "acctype" => account_type = Some(value),
                    "openid" => open_id = Some(value),
                    "ieg_ams_token" => ams_token = Some(value),
                    "ieg_ams_token_time" => ams_token_time = Some(value),
                    "ieg_ams_session_token" => ams_session_token = Some(value),
                    _ => (),
                }
            }
        }

        if let (
            Some(account_type),
            Some(open_id),
            Some(ams_token),
            Some(ams_token_time),
            Some(ams_session_token),
        ) = (
            account_type,
            open_id,
            ams_token,
            ams_token_time,
            ams_session_token,
        ) {
            Ok(Self {
                account_type: account_type.to_string(),
                open_id: open_id.to_string(),
                ams_token: ams_token.to_string(),
                ams_token_time: ams_token_time.to_string(),
                ams_session_token: ams_session_token.to_string(),
            })
        } else {
            Err(Error::InvalidCredentials)
        }
    }

    pub fn to_cookies(&self) -> String {
        format!(
            "acctype={};openid={};ieg_ams_token={};ieg_ams_token_time={};ieg_ams_session_token={}",
            self.account_type,
            self.open_id,
            self.ams_token,
            self.ams_token_time,
            self.ams_session_token,
        )
    }
}
