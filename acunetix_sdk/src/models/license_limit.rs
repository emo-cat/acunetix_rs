/*
 * AWVS12 client api
 *
 * Awvs12 client api [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/). 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LicenseLimit {
    #[serde(rename = "demo_targets", skip_serializing_if = "Option::is_none")]
    pub demo_targets: Option<i32>,
    #[serde(rename = "standard_targets", skip_serializing_if = "Option::is_none")]
    pub standard_targets: Option<i32>,
    #[serde(rename = "engines", skip_serializing_if = "Option::is_none")]
    pub engines: Option<i32>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<i32>,
}

impl LicenseLimit {
    pub fn new() -> LicenseLimit {
        LicenseLimit {
            demo_targets: None,
            standard_targets: None,
            engines: None,
            users: None,
        }
    }
}

