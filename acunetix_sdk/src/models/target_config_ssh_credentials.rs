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
pub struct TargetConfigSshCredentials {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl TargetConfigSshCredentials {
    pub fn new() -> TargetConfigSshCredentials {
        TargetConfigSshCredentials {
            kind: None,
        }
    }
}


