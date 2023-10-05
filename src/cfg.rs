use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;
use std::process::exit;

static HOST_KEY: &str = "host";
static PORT_KEY: &str = "port";
static ENV_PATH: &str = "./.env";

pub struct Config {
    host: String,
    port: String,
}

impl Config {
    pub fn get_addr(self) -> String {
        return format!("{}:{}", self.host, self.port);
    }
}

pub fn parse_envs() -> Result<Config, Error> {
    let envs: Vec<String> = read_envs();
    let mut cfg_map: HashMap<String, String> = HashMap::new();

    for line in envs {
        let eq_idx: usize = line.find("=").unwrap();
        cfg_map.insert(line[..eq_idx].to_string(), line[eq_idx + 1..].to_string());
    }

    if cfg_map.contains_key(&HOST_KEY.to_string()) && cfg_map.contains_key(&PORT_KEY.to_string()) {
        return Ok(Config {
            host: cfg_map.get(&HOST_KEY.to_string()).unwrap().to_string(),
            port: cfg_map.get(&PORT_KEY.to_string()).unwrap().to_string(),
        });
    } else {
        println!("Provide 'host' and 'port' variables in .env file.");
        exit(1);
    }
}

fn read_envs() -> Vec<String> {
    return read_to_string(ENV_PATH)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
