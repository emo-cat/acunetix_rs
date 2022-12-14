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
pub struct TargetDel {
    #[serde(rename = "target_deletion_consumed", skip_serializing_if = "Option::is_none")]
    pub target_deletion_consumed: Option<bool>,
    #[serde(rename = "target_deletion_allowance", skip_serializing_if = "Option::is_none")]
    pub target_deletion_allowance: Option<i64>,
}

impl TargetDel {
    pub fn new() -> TargetDel {
        TargetDel {
            target_deletion_consumed: None,
            target_deletion_allowance: None,
        }
    }
}


