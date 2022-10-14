use crate::{parse_proc_loadavg, AgentErrors, ZabbixValue};
use std::string::String;

pub fn get_zabbix_value(val: ZabbixValue) -> Result<String, AgentErrors> {
    let loadavg = parse_proc_loadavg().expect("Error when getting the loadavg!");
    let zabbix_result_val: String;
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
    }
    Ok(zabbix_result_val)
}
