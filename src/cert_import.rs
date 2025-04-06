use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};

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
                CertStoreLocation = 'Cert:\LocalMachine\My'
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