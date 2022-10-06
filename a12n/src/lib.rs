#![allow(clippy::derive_partial_eq_without_eq)]

use yaserde_derive::*;
use sha1::{Sha1, Digest};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct Password {
    #[yaserde(attribute, rename = "Type")]
    pub password_type: String,

    #[yaserde(text)]
    pub content: String
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsse",
    namespace = "wsse: http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd",
    namespace = "wsu: http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd"
)]
pub struct UsernameToken {

    #[yaserde(rename = "Username", prefix = "wsse")]
    pub username: String,

    #[yaserde(rename = "Password", prefix = "wsse")]
    pub password: Password,

    #[yaserde(rename = "Nonce", prefix = "wsse")]
    pub nonce: String,

    #[yaserde(rename = "Created", prefix = "wsu")]
    pub created: String
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsse",
    namespace = "wsse: http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd",
    namespace = "wsu: http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd"
)]
pub struct Security {

    #[yaserde(rename = "UsernameToken", prefix = "wsse")]
    pub username_token: UsernameToken
}

const PASSWORD_DIGEST: &str = "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest";

impl Security {

    // Return true if this Authentication Request is a password digest request,
    // and the encoded password matches the given argument. NB: does not authenticate
    // the username.
    pub fn is_password_authentic(&self, password: &str) -> bool {

        if self.username_token.password.password_type != PASSWORD_DIGEST {
            return false;
        }

        //1. b64 decode the nonce
        let nonce = match base64::decode(&self.username_token.nonce) {
            Ok(n) => n,
            Err(_) => return false
        };

        //2. Recreate the SHA1 hash from (nonce, data, password)
        let digest_str = {
            let mut hasher = Sha1::new();
            hasher.update(nonce);
            hasher.update(self.username_token.created.as_bytes());
            hasher.update(password.as_bytes());
            base64::encode(hasher.finalize())
        };

        //3. Compare the calculated and given password-digest
        digest_str == self.username_token.password.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const XML: &str = r#"
        <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
            <wsse:UsernameToken>
                <wsse:Username>admin</wsse:Username>
                <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">Ab75bIiEIr3MO6PGSEQCZE/SOMw=</wsse:Password>
                <wsse:Nonce>9V+1xELAOjEaROQdkDYG1g==</wsse:Nonce>
                <wsu:Created>2022-03-07T13:19:55Z</wsu:Created>
            </wsse:UsernameToken>
        </wsse:Security>"#;

    #[test]
    fn test_simple_parse() {
        let security: Security = yaserde::de::from_str(&XML).unwrap();

        assert_eq!(security.username_token.username, "admin");
        assert_eq!(security.username_token.password.password_type, "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest");
        assert_eq!(security.username_token.password.content, "Ab75bIiEIr3MO6PGSEQCZE/SOMw=");
        assert_eq!(security.username_token.nonce, "9V+1xELAOjEaROQdkDYG1g==");
        assert_eq!(security.username_token.created, "2022-03-07T13:19:55Z");
    }

    #[test]
    fn test_is_authentic() {
        let security: Security = yaserde::de::from_str(&XML).unwrap();

        assert_eq!(security.username_token.username, "admin");
        assert!(security.is_password_authentic("password123"));
    }

}
