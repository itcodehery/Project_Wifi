use color_eyre::eyre::Result;
use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct WifiInfo {
    pub ssid: String,
    pub network_type: String,
    pub authentication: String,
    pub encryption: String,
}

pub fn get_connected_ssid() -> Result<Option<String>> {
    let output = Command::new("netsh")
        .args(["wlan", "show", "interfaces"])
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("SSID") {
            if let Some(ssid) = trimmed_line.split(':').nth(1) {
                let ssid = ssid.trim().to_string();
                if !ssid.is_empty() {
                    return Ok(Some(ssid));
                }
            }
        }
    }

    Ok(None)
}

pub fn get_wifi_networks() -> Result<Vec<WifiInfo>> {
    let output = Command::new("netsh")
        .args(["wlan", "show", "networks"])
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut wifi_list = Vec::new();
    for network_part in output_str.split("SSID").skip(1) {
        let mut wifi_info = WifiInfo::default();
        let mut lines = network_part.lines();
        if let Some(first_line) = lines.next() {
            if let Some(ssid) = first_line.split(':').nth(1) {
                wifi_info.ssid = ssid.trim().to_string();
            }
        }

        for line in lines {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("Network type") {
                if let Some(ntype) = trimmed_line.split(':').nth(1) {
                    wifi_info.network_type = ntype.trim().to_string();
                }
            } else if trimmed_line.starts_with("Authentication") {
                if let Some(auth) = trimmed_line.split(':').nth(1) {
                    wifi_info.authentication = auth.trim().to_string();
                }
            } else if trimmed_line.starts_with("Encryption") {
                if let Some(enc) = trimmed_line.split(':').nth(1) {
                    wifi_info.encryption = enc.trim().to_string();
                }
            }
        }
        if !wifi_info.ssid.is_empty() {
            wifi_list.push(wifi_info);
        }
    }

    Ok(wifi_list)
}

pub fn connect_with_password(ssid: &str, password: &str) -> Result<()> {
    let profile_xml = format!(
        r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <name>{}</name>
        </SSID>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>auto</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>WPA2PSK</authentication>
                <encryption>AES</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
            <sharedKey>
                <keyType>passPhrase</keyType>
                <protected>false</protected>
                <keyMaterial>{}</keyMaterial>
            </sharedKey>
        </security>
    </MSM>
</WLANProfile>"#,
        ssid, ssid, password
    );

    let profile_path = format!("{}.xml", ssid);
    std::fs::write(&profile_path, profile_xml)?;

    Command::new("netsh")
        .args([
            "wlan",
            "add",
            "profile",
            &format!("filename={}", profile_path),
        ])
        .output()?;

    Command::new("netsh")
        .args(["wlan", "connect", &format!("name={}", ssid)])
        .output()?;

    std::fs::remove_file(&profile_path)?;

    Ok(())
}

pub fn run_speed_test() -> Result<String> {
    let url = "http://speed.hetzner.de/10MB.bin";

    let output = Command::new("cmd")
        .args(&[
            "/c",
            "curl",
            "-o",
            "NUL",
            "-s",
            "-w",
            "%{speed_download}",
            url,
        ])
        .output()?;

    if !output.status.success() {
        return Err(color_eyre::eyre::eyre!("curl command failed"));
    }

    let curl_output = String::from_utf8_lossy(&output.stdout);
    let trimmed_output = curl_output.trim();
    let replaced_output = trimmed_output.replace(',', ".");

    match replaced_output.parse::<f64>() {
        Ok(speed_bytes_per_sec) => {
            let speed_mbps = (speed_bytes_per_sec * 8.0) / 1_000_000.0;
            Ok(format!("{:.2} Mbps", speed_mbps * 100.0))
        }
        Err(_) => Err(color_eyre::eyre::eyre!(
            "Invalid response: '{}'",
            trimmed_output
        )),
    }
}
