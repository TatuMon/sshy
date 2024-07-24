use std::{
    fs::File,
    io::{self, BufRead, BufReader}, path::PathBuf,
};

use color_eyre::eyre::{Context, Result};

fn get_file_lines(file: &File) -> io::Lines<BufReader<&File>> {
    let reader = io::BufReader::new(file);
    reader.lines()
}

fn get_file_hostnames(known_hosts_file: &File) -> Vec<String> {
    let mut known_hosts: Vec<String> = vec!();

    for line in get_file_lines(known_hosts_file) {
        if let Ok(line) = line {
            if let Some(hostname) = line.split_once(' ').map(|s| s.0.to_owned()) {
                if !known_hosts.contains(&hostname) {
                    known_hosts.push(hostname)
                }
            }
        } else {
            continue
        }
    }

    known_hosts
}

pub fn get_known_hosts() -> Result<Vec<String>> {
    let mut known_hosts: Vec<String> = vec!();

    let user_known_hosts_filename = dirs::home_dir().unwrap_or_default().join(".ssh/known_hosts");
    if user_known_hosts_filename.exists() {
        let user_file = File::open(user_known_hosts_filename).wrap_err("error reading file")?;
        known_hosts.append(&mut get_file_hostnames(&user_file));
    }

    let system_known_hosts_filename = PathBuf::from("/etc/ssh/ssh_known_hosts");
    if system_known_hosts_filename.exists() {
        let system_file = File::open(system_known_hosts_filename).wrap_err("error reading file")?;
        known_hosts.append(&mut get_file_hostnames(&system_file));
    }

    Ok(known_hosts)
}

