/*
 * Acunetix12 client api
 *
 * Acunetix12 client api [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/). 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanStat {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "scanning_app", skip_serializing_if = "Option::is_none")]
    pub scanning_app: Option<Box<crate::models::ScanApp>>,
    #[serde(rename = "serverity_counts", skip_serializing_if = "Option::is_none")]
    pub serverity_counts: Option<Box<crate::models::ScanStatServerityCounts>>,
}

impl ScanStat {
    pub fn new() -> ScanStat {
        ScanStat {
            status: None,
            scanning_app: None,
            serverity_counts: None,
        }
    }
}


