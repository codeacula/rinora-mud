use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
    sync::Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbSystemInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub administrator_email: String,
    pub version: i32,
}

pub struct SystemRepo {
    pub database: Database,
    pub system: Collection<DbSystemInfo>,
}

impl SystemRepo {
    pub fn new(database: &Database) -> Self {
        let system = database.collection::<DbSystemInfo>("system");

        SystemRepo {
            database: database.clone(),
            system,
        }
    }

    pub fn get_system_config(&self) -> Result<DbSystemInfo, String> {
        let system_info_res = self.system.find_one(doc! {}, None);

        let system_info = match system_info_res {
            Ok(system_info) => system_info,
            Err(e) => {
                return Err(format!(
                    "Error while trying to get system information: {:?}",
                    e
                ));
            }
        };

        if system_info.is_some() {
            return Ok(system_info.unwrap());
        }

        let new_info = DbSystemInfo {
            administrator_email: "codeacula@codeacula.com".to_string(),
            id: None,
            version: 0,
        };

        if let Err(e) = self.system.insert_one(&new_info, None) {
            return Err(format!(
                "Error trying to add new system information: {:?}",
                e
            ));
        }

        Ok(new_info)
    }
}
