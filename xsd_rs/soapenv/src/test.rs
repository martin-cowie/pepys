use super::*;

const XML :&str = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">RuCo1C7tt98WWgeHm916EpZVFf4=</wsse:Password>
                        <wsse:Nonce>n/zuuhbyNflhWos8TbY2Vg==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:45Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetSnapshotUri xmlns="http://www.onvif.org/ver10/media/wsdl">
                    <ProfileToken>profile_token</ProfileToken>
                </GetSnapshotUri>
            </s:Body>
        </s:Envelope>"#;

#[test]
fn do_test() {

    let env: Envelope = yaserde::de::from_str(&XML).unwrap();

    assert_eq!(env.header.unwrap().security.unwrap().username_token.username, "admin");
}