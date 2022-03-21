use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Backends {
    pub values:HashMap<String,Backend>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Backend {
    pub fns:Vec<BackendItem>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct  BackendItem {
    pub name:String,
    #[serde(rename = "type")]
    pub typ:String
}



impl Backends {
    pub fn new() -> Self {
        let backends_json = include_str!("backends.json");
        match serde_json::from_str::<HashMap<String,Backend>>(backends_json) {
            Ok(values) => Backends { values },
            Err(err) => {
                log::error!("read backends.json error: {:?}",err);
                Backends {values:HashMap::default() }
            }
        }
    }
}