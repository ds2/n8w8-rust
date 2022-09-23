use crate::errors::DataStoreError;
use log::debug;
use nachtwacht_models::n8w8::ProcStatCpu;
use std::fs::File;
use std::io::{BufReader, Read};

#[cfg(unix)]
pub fn parse_proc_stat() -> Result<Vec<ProcStatCpu>, DataStoreError> {
    let mut str = String::new();
    let file = File::open("/proc/stat").expect("Error in reading /proc/stat");
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_string(&mut str)
        .expect("Unable to read line");
    let mut cpuvec: Vec<ProcStatCpu> = Vec::new();
    let mut cpu_id = 0;
    for s in str.lines().filter(|&s| s.starts_with("cpu")) {
        debug!("cpu line: {}", s);
        let mut line_vals = s.split_whitespace();
        line_vals.next(); //the "cpu" header
        let psc = ProcStatCpu {
            // the first line is an aggregation of all following cpu cores.
            id: cpu_id,
            user: line_vals.next().unwrap().parse::<u32>().unwrap(),
            system: line_vals.next().unwrap().parse::<u32>().unwrap(),
            nice: line_vals.next().unwrap().parse::<u32>().unwrap(),
            idle: line_vals.next().unwrap().parse::<u32>().unwrap(),
            iowait: line_vals.next().unwrap().parse::<u32>().unwrap(),
            irq: line_vals.next().unwrap().parse::<u32>().unwrap(),
            softirq: line_vals.next().unwrap().parse::<u32>().unwrap(),
            steal: line_vals.next().unwrap().parse::<u32>().unwrap(),
            special_fields: Default::default(),
        };
        cpuvec.push(psc);
        cpu_id += 1;
    }
    Ok(cpuvec)
}

#[cfg(test)]
mod tests {
    use crate::parse_proc_stat;

    #[test]
    fn it_works() {
        let result = parse_proc_stat();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.len() > 1);
        println!("Result: {:?}", result);
    }
}
