use crate::errors::AgentErrors;
use log::debug;
use nachtwacht_models::n8w8::ProcMemInfo;
use std::fs::File;
use std::io::{BufReader, Read};

#[cfg(target_os = "linux")]
pub fn parse_proc_mem_info() -> Result<ProcMemInfo, AgentErrors> {
    debug!("Starting check for /proc/meminfo..");
    let mut str = String::new();
    let file = File::open("/proc/meminfo").expect("Error in reading /proc/meminfo");
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_string(&mut str)
        .expect("Unable to read line");
    debug!("We got these lines: {}", str);
    let lines = str.lines();
    let mut mem_info = ProcMemInfo {
        MemTotal: 0,
        MemFree: 0,
        MemAvailable: 0,
        Buffers: 0,
        Cached: 0,
        SwapCached: 0,
        SwapTotal: 0,
        SwapFree: 0,
        special_fields: Default::default(),
    };
    for this_line in lines {
        debug!("Line to parse is: {}", this_line);
        let mut splits = this_line.split_whitespace();
        match splits.next().unwrap() {
            "MemTotal:" => {
                mem_info.MemTotal = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "MemFree:" => {
                mem_info.MemFree = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "MemAvailable:" => {
                mem_info.MemAvailable = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "Buffers:" => {
                mem_info.Buffers = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "Cached:" => mem_info.Cached = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024,
            "SwapCached:" => {
                mem_info.SwapCached = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "SwapTotal:" => {
                mem_info.SwapTotal = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "SwapFree:" => {
                mem_info.SwapFree = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            &_ => {}
        }
    }
    debug!("Returning this val: {}", mem_info);
    Ok(mem_info)
}

#[cfg(unix)]
pub fn parse_proc_mem_info() -> Result<ProcMemInfo, AgentErrors> {
    Err(AgentErrors::NotImplemented())
}

#[cfg(test)]
mod tests {
    use crate::proc_meminfo::parse_proc_mem_info;

    #[test_log::test]
    #[cfg(target_os = "linux")]
    fn it_works() {
        let result = parse_proc_mem_info();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.MemFree > 0);
        println!("Result: {:?}", result);
    }
}
