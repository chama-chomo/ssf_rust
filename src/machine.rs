use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Machine {
    pub ip: Option<String>,
    pub sp_ip: Option<String>,
    pub mac: Option<String>,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub operatingsystem_name: Option<String>,
    pub hostgroup_name: Option<String>,
    pub owner_name: Option<String>,
    pub model_name: Option<String>,
    pub global_status_label: Option<String>,
    pub build_status_label: Option<String>,
}

pub struct HostDetails {
    details: String,
}

impl Machine {
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("n/a")
    }

    pub fn display_group(&self) -> &str {
        self.hostgroup_name.as_deref().unwrap_or("n/a")
    }

    pub fn display_host_details(&self) -> String {
        let hdetails: String = format!(
            "SELECTION DETAILS
-----------------
IP:             {}
ILO IP:         {}
MAC ADD:        {}
MODEL:          {}
OS:             {}
GROUP ASSIGNED: {}
OWNER:          {}
GLOBAL STATUS:  {}
BUILD STATUS:   {}
COMMENT:        {}
",
            self.ip.as_ref().unwrap_or(&"n/a".to_string()),
            self.sp_ip.as_ref().unwrap_or(&"n/a".to_string()),
            self.mac.as_ref().unwrap_or(&"n/a".to_string()),
            self.model_name.as_ref().unwrap_or(&"n/a".to_string()),
            self.operatingsystem_name
                .as_ref()
                .unwrap_or(&"n/a".to_string()),
            self.hostgroup_name.as_ref().unwrap_or(&"n/a".to_string()),
            self.owner_name.as_ref().unwrap_or(&"n/a".to_string()),
            self.global_status_label
                .as_ref()
                .unwrap_or(&"n/a".to_string()),
            self.build_status_label
                .as_ref()
                .unwrap_or(&"n/a".to_string()),
            self.comment.as_ref().unwrap_or(&"n/a".to_string()),
        );

        // let hd: HostDetails = HostDetails { details: hdetails };
        hdetails
    }
}

#[derive(Debug, Deserialize)]
pub struct HostsArray {
    pub results: Vec<Machine>,
}

impl HostsArray {
    pub fn create_array(json_hosts: &str) -> Result<HostsArray, Error> {
        let hosts_arr: HostsArray = serde_json::from_str(json_hosts)?;
        Ok(hosts_arr)
    }
}
