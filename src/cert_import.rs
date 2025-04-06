use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};

pub fn check_thumbprint_in_cert_store(thumbprint: &str) -> std::io::Result<bool> {
    let ps_script = r#"
        Get-ChildItem Cert:\LocalMachine\My |
        Select-Object -ExpandProperty Thumbprint |
        ForEach-Object { $_.ToUpper().Replace(" ", "") }
    "#;

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(ps_script)
        .output()?;

    if !output.status.success() {
        eprintln!("PowerShell 执行失败：{}", String::from_utf8_lossy(&output.stderr));
        return Ok(false);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let thumbprint_clean = thumbprint.to_uppercase().replace(" ", "");

    let found = stdout.lines().any(|line| line.trim() == thumbprint_clean);
    Ok(found)
}

pub fn import_pfx(pfx_path: &PathBuf, password: &str) -> Result<()> {
    // Validate the PFX file exists
    if !pfx_path.exists() {
        return Err(anyhow::anyhow!("PFX file not found at {}", pfx_path.display()));
    }

    // Escape the password for PowerShell
    let escaped_password = format!("(ConvertTo-SecureString -String '{}' -AsPlainText -Force)",
                                   password.replace("'", "''"));

    let cmd = format!(
        r#"
        try {{
            $params = @{{
                FilePath = '{}'
                CertStoreLocation = 'Cert\My'
                Password = {}
            }}
            Import-PfxCertificate @params -ErrorAction Stop
        }} catch {{
            Write-Error $_.Exception.Message
            exit 1
        }}
        "#,
        pfx_path.display().to_string().replace("'", "''"),
        escaped_password
    );

    let output = Command::new("powershell")
        .args(["-Command", &cmd])
        .output()
        .context("Failed to execute PowerShell command")?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Failed to import PFX: {}",
            error_msg.trim()
        ));
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub fn get_cert_thumbprint(cert_path: &str) -> Result<String> {
    // 使用 openssl x509 命令从 PEM 文件中提取证书的 SHA1 thumbprint
    let output = Command::new("openssl")
        .args(["x509", "-in", &*cert_path.to_string(), "-fingerprint", "-sha1", "-noout"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("无法计算 thumbprint"));
    }

    // 提取 thumbprint 字符串
    let thumbprint_str = String::from_utf8_lossy(&output.stdout);
    let thumbprint = thumbprint_str
        .trim()
        .replace("sha1 Fingerprint=", "")
        .replace(":", "")
        .to_uppercase();

    Ok(thumbprint)
}