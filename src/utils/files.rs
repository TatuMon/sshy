use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use color_eyre::eyre::{Context, Result};

pub fn get_known_hosts() -> Result<Vec<String>> {
    let mut known_hosts: Vec<String> = vec![];

    let user_known_hosts_filename = dirs::home_dir()
        .unwrap_or_default()
        .join(".ssh/known_hosts");
    if user_known_hosts_filename.exists() {
        let user_file = File::open(user_known_hosts_filename).wrap_err("error reading file")?;
        known_hosts.append(&mut get_file_hostnames(&user_file));
    }

    let system_known_hosts_filename = PathBuf::from("/etc/ssh/ssh_known_hosts");
    if system_known_hosts_filename.exists() {
        let system_file = File::open(system_known_hosts_filename).wrap_err("Error reading file")?;
        known_hosts.append(&mut get_file_hostnames(&system_file));
    }

    Ok(known_hosts)
}

pub fn get_public_keys_names() -> Result<Vec<String>> {
    let mut pub_keys_names: Vec<String> = vec![];
    let ssh_dir = dirs::home_dir().unwrap_or_default().join(".ssh/");

    if ssh_dir.exists() && ssh_dir.is_dir() {
        for entry in ssh_dir.read_dir().wrap_err("Error reading directory")? {
            if let Ok(entry) = entry {
                let entry_name = entry.file_name().to_string_lossy().to_string();
                if entry_name.ends_with(".pub") {
                    pub_keys_names.push(entry_name)
                }
            } else {
                continue;
            }
        }
    }

    Ok(pub_keys_names)
}

fn get_file_lines(file: &File) -> io::Lines<BufReader<&File>> {
    let reader = io::BufReader::new(file);
    reader.lines()
}

fn get_file_hostnames(known_hosts_file: &File) -> Vec<String> {
    let mut known_hosts: Vec<String> = vec![];

    for line in get_file_lines(known_hosts_file) {
        if let Ok(line) = line {
            if let Some(hostname) = line.split_once(' ').map(|s| s.0.to_owned()) {
                if !known_hosts.contains(&hostname) {
                    known_hosts.push(hostname)
                }
            }
        } else {
            continue;
        }
    }

    known_hosts
}

pub fn get_user_ssh_dir() -> Result<PathBuf> {
    let path = PathBuf::from(std::env::var("HOME").wrap_err("Couldn't find home directory")?)
        .join(".ssh/");

    Ok(path)
}

pub fn delete_key_pair(private_key_name: &str) -> Result<()> {
    let ssh_dir = get_user_ssh_dir()?;

    let private_key_path = ssh_dir.clone().join(private_key_name);

    let public_key_name = format!("{}.pub", private_key_name);
    let public_key_path = ssh_dir.join(public_key_name.clone());

    if private_key_path.exists() {
        fs::remove_file(private_key_path).wrap_err(format!("Failed to remove private key '{}'", private_key_name.to_owned()))?;
    }
    if public_key_path.exists() {
        fs::remove_file(public_key_path).wrap_err(format!("Failed to remove public key '{}'", public_key_name))?;
    }


    return Ok(())
}

pub fn get_pub_key_content(key_name: &str) -> Result<String> {
    let ssh_dir = get_user_ssh_dir()?;
    let mut key_path = ssh_dir.join(key_name);
    
    match key_path.extension() {
        None => {
            key_path.set_extension(".pub");
        }
        Some(ext) => {
            if ext != "pub" {
                key_path.set_extension(".pub");
            }
        }
    };

    fs::read_to_string(key_path).wrap_err("Failed to read public key content")
}

pub fn get_client_config_content() -> Result<String> {
    let ssh_dir = get_user_ssh_dir()?;
    let config_file_path = ssh_dir.join("config");

    fs::read_to_string(config_file_path).wrap_err("Failed to read client config file")
}
