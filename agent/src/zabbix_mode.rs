use std::string::String;

use crate::AgentErrors::FailedToGetLocalInfo;
use crate::{parse_proc_loadavg, parse_proc_stat, AgentErrors, ZabbixValue};

pub fn get_zabbix_value(val: ZabbixValue) -> Result<String, AgentErrors> {
    let loadavg = parse_proc_loadavg().expect("Error when getting the loadavg!");
    let mut zabbix_result_val: String = "".to_string();
    //request for zabbix mode
    match val {
        ZabbixValue::Load1 => {
            zabbix_result_val = loadavg.load1.to_string();
        }
        ZabbixValue::Load5 => {
            zabbix_result_val = loadavg.load5.to_string();
        }
        ZabbixValue::Load15 => {
            zabbix_result_val = loadavg.load15.to_string();
        }
        ZabbixValue::IoWait => {
            let proc_stats = parse_proc_stat().unwrap();
            if proc_stats.len() > 0 {
                zabbix_result_val = proc_stats.first().unwrap().iowait.to_string();
            }
        }
    }
    if zabbix_result_val.len() > 0 {
        Ok(zabbix_result_val)
    } else {
        Err(FailedToGetLocalInfo())
    }
}
