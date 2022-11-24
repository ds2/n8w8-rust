use clap::ValueEnum;
use nachtwacht_core::errors::AgentErrors;
use nachtwacht_core::errors::AgentErrors::FailedToGetLocalInfo;
use nachtwacht_core::proc_loadavg::parse_proc_loadavg;
use nachtwacht_core::proc_meminfo::parse_proc_mem_info;
use nachtwacht_core::proc_stat::parse_proc_stat;
use std::string::String;

/// This enum contains all the values that Zabbix may need.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ZabbixValue {
    Load1,
    Load5,
    Load15,
    IoWait,
    MemFreePercent,
    MemFree,
}

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
        ZabbixValue::MemFreePercent => {
            let mem_info =
                parse_proc_mem_info().expect("Error when getting the proc/meminfo details!");
            let perc_val: f64 = 100.0 * mem_info.MemFree as f64 / mem_info.MemTotal as f64;
            zabbix_result_val = perc_val.to_string();
        }
        ZabbixValue::MemFree => {
            let mem_info =
                parse_proc_mem_info().expect("Error when getting the proc/meminfo details!");
            zabbix_result_val = mem_info.MemFree.to_string();
        }
    }
    if zabbix_result_val.len() > 0 {
        Ok(zabbix_result_val)
    } else {
        Err(FailedToGetLocalInfo("/proc/meminfo".to_string()))
    }
}