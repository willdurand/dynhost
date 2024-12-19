use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::env;
use std::process;

#[cfg(test)]
use mockito;

macro_rules! ensure_env {
    ($var:expr) => {
        match env::var($var) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("{} must be set", $var);
                process::exit(1);
            }
        }
    };
}

fn update_dynhost(
    app_version: String,
    username: String,
    password: String,
    hostname: String,
) -> Result<String, String> {
    #[cfg(not(test))]
    let get_ip_url = "https://ipv4.icanhazip.com";
    #[cfg(test)]
    let get_ip_url = &mockito::server_url();

    #[cfg(not(test))]
    let ovh_url = "http://www.ovh.com/nic/update";
    #[cfg(test)]
    let ovh_url = &mockito::server_url();

    let client = Client::new();
    match client.get(get_ip_url).send() {
        Ok(get_ip) => {
            if !get_ip.status().is_success() {
                return Err("could not retrieve the current IP".into());
            }

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
                    "could not update the DynHOST (status = {:?})",
                    update_dns.status()
                ))
            }
        }
        Err(err) => Err(format!("could not reach the IP service ({:?})", err)),
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

    let username = ensure_env!("DYNHOST_USERNAME");
    let password = ensure_env!("DYNHOST_PASSWORD");
    let hostname = ensure_env!("DYNHOST_HOSTNAME");

    match update_dynhost(app_version, username, password, hostname) {
        Ok(ip) => {
            println!("Successfully updated DynHOST with IP: {}", ip);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn it_returns_the_updated_ip_on_success() {
        let _ = env_logger::try_init();

        let app_version = env!("CARGO_PKG_VERSION").into();

        let username = "user".to_string();
        let password = "pass".to_string();
        let hostname = "host".to_string();

        let ip = "1.1.1.1";
        let _m1 = mock("GET", "/").with_status(200).with_body(ip).create();

        let url = format!("/?system=dyndns&hostname={}&myip={}", hostname, ip);
        let _m2 = mock("GET", url.as_str()).with_status(200).create();

        let res = update_dynhost(app_version, username, password, hostname);
        assert_eq!(Ok(ip.into()), res);
    }

    #[test]
    fn it_returns_an_error_when_get_ip_fails() {
        let _ = env_logger::try_init();

        let app_version = env!("CARGO_PKG_VERSION").into();

        let username = "user".to_string();
        let password = "pass".to_string();
        let hostname = "host".to_string();

        let _m = mock("GET", "/").with_status(400).create();

        let res = update_dynhost(app_version, username, password, hostname);
        assert_eq!(Err("could not retrieve the current IP".into()), res);
    }

    #[test]
    fn it_returns_an_error_when_update_fails() {
        let _ = env_logger::try_init();

        let app_version = env!("CARGO_PKG_VERSION").into();

        let username = "user".to_string();
        let password = "pass".to_string();
        let hostname = "host".to_string();

        let ip = "1.1.1.1";
        let _m1 = mock("GET", "/").with_status(200).with_body(ip).create();

        let url = format!("/?system=dyndns&hostname={}&myip={}", hostname, ip);
        let _m2 = mock("GET", url.as_str()).with_status(400).create();

        let res = update_dynhost(app_version, username, password, hostname);
        assert_eq!(
            Err("could not update the DynHOST (status = 400)".into()),
            res
        );
    }
}
