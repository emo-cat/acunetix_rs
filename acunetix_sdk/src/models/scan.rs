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
pub struct Scan {
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "profile_id")]
    pub profile_id: String,
    #[serde(rename = "ui_sessionid", skip_serializing_if = "Option::is_none")]
    pub ui_sessionid: Option<String>,
    #[serde(rename = "schedule")]
    pub schedule: Box<crate::models::Schedual>,
}

impl Scan {
    pub fn new(target_id: String, profile_id: String, schedule: crate::models::Schedual) -> Scan {
        Scan {
            target_id,
            profile_id,
            ui_sessionid: None,
            schedule: Box::new(schedule),
        }
    }
}


