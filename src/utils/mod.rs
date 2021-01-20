use rusoto_core::{credential::ProvideAwsCredentials, Region, RusotoError};
use rusoto_sts::{Sts, StsClient};
use rusoto_sts::{AssumeRoleRequest, GetSessionTokenRequest};

pub async fn main(){
    let sts = StsClient::new(Region::ApNortheast1);
    let get_session_token_res = sts.get_session_token(GetSessionTokenRequest{
        token_code: Some("12345".to_owned()),
        serial_number: Some("12345".to_owned()),
        ..Default::default()
    })
    .await;

    match get_session_token_res {
        Err(RusotoError::Unknown(http_res)) => {
            let msg = ::std::str::from_utf8(&http_res.body).unwrap();
            assert!(msg.contains(
                    "Please verify your MFA serial number is valid and associated with this user."
            ))
        }

    _ => panic!("this should have been an Unknown STS Error: {:?}", get_session_token_res)
    }
}
