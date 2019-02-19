use reqwest::header::USER_AGENT;
use reqwest::Client;
use std::env;
use std::process;

macro_rules! ensure_env {
    ($var:expr) => {
        match env::var($var) {
            Ok(value) => value,
            Err(_) => {
                return Err(format!("{} must be set", $var));
            }
        };
    };
}

fn update_dynhost(app_version: String) -> Result<String, String> {
    let username = ensure_env!("DYNHOST_USERNAME");
    let password = ensure_env!("DYNHOST_PASSWORD");
    let hostname = ensure_env!("DYNHOST_HOSTNAME");

    let get_ip_url = "https://ipv4.icanhazip.com";
    let ovh_url = "http://www.ovh.com/nic/update";

    let client = Client::new();
    match client.get(get_ip_url).send() {
        Ok(mut get_ip) => {
            let my_ip = get_ip.text().expect("invalid or no IP retrieved");

            let update_dns = client
                .get(ovh_url)
                .query(&[
                    ("system", "dyndns"),
                    ("hostname", &hostname),
                    ("myip", &my_ip),
                ])
                .basic_auth(username, Some(password))
                .header(USER_AGENT, format!("dynhost/{}", app_version))
                .send()
                .unwrap();

            if update_dns.status().is_success() {
                Ok(my_ip)
            } else {
                Err(format!(
                    "could not update the DynHOST: {:?}",
                    update_dns.status()
                ))
            }
        }
        Err(err) => Err(format!("could not retrieve the current IP ({})", err)),
    }
}

fn main() {
    let app_version = env!("CARGO_PKG_VERSION").into();

    if let Some(first_arg) = env::args().nth(1) {
        if first_arg == "-V" {
            println!("{}", app_version);
            process::exit(0);
        }
    }

    match update_dynhost(app_version) {
        Ok(ip) => {
            println!("Successfully updated DynHOST with IP: {}", ip);
        },
        Err(err) => {
            eprintln!("ERROR: {}", err);
            process::exit(1);
        }
    }
}
