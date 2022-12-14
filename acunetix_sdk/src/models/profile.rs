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
pub struct Profile {
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "profile_id", skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "sort_order", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl Profile {
    pub fn new() -> Profile {
        Profile {
            custom: None,
            name: None,
            profile_id: None,
            sort_order: None,
        }
    }
}


