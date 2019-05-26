use crate::oautherror::OAuthError;
use from_to_file::*;
use graph_error::GraphFailure;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, FromToFile)]
#[serde(rename = "id_token")]
pub struct IdToken {
    code: Option<String>,
    id_token: String,
    state: Option<String>,
    session_state: Option<String>,
}

impl IdToken {
    pub fn new(id_token: &str, code: &str, state: &str, session_state: &str) -> IdToken {
        IdToken {
            code: Some(code.into()),
            id_token: id_token.into(),
            state: Some(state.into()),
            session_state: Some(session_state.into()),
        }
    }

    pub fn id_token(&mut self, id_token: &str) {
        self.id_token = id_token.into();
    }

    pub fn code(&mut self, code: &str) {
        self.code = Some(code.into());
    }

    pub fn state(&mut self, state: &str) {
        self.state = Some(state.into());
    }

    pub fn session_state(&mut self, session_state: &str) {
        self.session_state = Some(session_state.into());
    }

    pub fn get_id_token(&self) -> String {
        self.id_token.clone()
    }

    pub fn get_code(&self) -> Option<String> {
        self.code.clone()
    }

    pub fn get_state(&self) -> Option<String> {
        self.state.clone()
    }

    pub fn get_session_state(&self) -> Option<String> {
        self.session_state.clone()
    }
}

impl TryFrom<String> for IdToken {
    type Error = OAuthError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id_token: IdToken = IdToken::from_str(value.as_str())?;
        Ok(id_token)
    }
}

impl TryFrom<&str> for IdToken {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id_token: IdToken = IdToken::from_str(value)?;
        Ok(id_token)
    }
}

impl FromStr for IdToken {
    type Err = GraphFailure;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<&str> = s.split_terminator('&').collect();
        if vec.is_empty() {
            return OAuthError::invalid_data("Could not parse string into id_token");
        }

        let mut id_token = IdToken::default();
        for string in vec.iter() {
            if string.starts_with("code=") {
                id_token.code(&string[5..]);
            } else if string.starts_with("id_token=") {
                id_token.id_token(&string[9..]);
            } else if string.starts_with("state=") {
                id_token.state(&string[6..]);
            } else if string.starts_with("session_state=") {
                id_token.session_state(&string[14..]);
            }
        }
        Ok(id_token)
    }
}
