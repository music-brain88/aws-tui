use rusoto_core::Region;
use rusoto_sts::Credentials;


fn get_sts() {
    let client = Credentials::new(Region::ApNortheast1);
}
