use std::{collections::HashMap, error::Error, fs::File, io::{self, BufRead}, path::Path};
use regex::Regex;
use serde_derive::Deserialize;

//data是string数组
#[derive(Deserialize, Debug)]
struct IPInfo {
    ret:String,
    data:  Vec<String>,
    ip: String,
}
fn main() {
    println!("Hello, world!");
    let reg_exp = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";
    let re = Regex::new(reg_exp).unwrap();

    if let Ok(lines) = read_lines("./access.log") {
        let mut ip_counter = HashMap::new();
        for line in lines.flatten() {
            if let Some(ip) = get_ip(&line, &re) {
                // println!("Line {}: {}", line_num, ip);
                let count = ip_counter.entry(ip).or_insert(0);
                *count += 1;
            }
        }
        let (max_ip, max_count) = calc_most_common_ip(&ip_counter);
        if let Ok(res) = request_ip_address(&max_ip) {
            println!("The most common ip is: {}, count: {}, location: {}",res.ip, max_count, res.data.join(","));
        }
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_ip(string: &str, re: &Regex) -> Option<String> {
    if let Some(cap) = re.captures(string) {
        return Some(cap[0].to_string());
    }
    None
}
fn calc_most_common_ip(ip_counter: &HashMap<String, u32>) -> (String, u32) {
    let mut max_ip = String::new();
    let mut max_count = 0;
    for (ip, count) in ip_counter {
        if *count > max_count {
            max_count = *count;
            max_ip = ip.clone();
        }
    }
    (max_ip, max_count)
}

fn request_ip_address(ip: &str) -> Result<IPInfo, Box<dyn Error>> {
    let url = format!("http://www.inte.net/tool/ip/api.ashx?ip={}&datatype=json", ip);

    let resp = reqwest::blocking::get(&url)?.text()?;
    let ip_info: IPInfo = serde_json::from_str(&resp)?;
    println!("-----------------");
    println!("{:#?}", ip_info);
    println!("-----------------");
    if(ip_info.ret != "ok"){
        return Err("ip_info.ret != ok".into());
    }
    Ok(ip_info)
}
