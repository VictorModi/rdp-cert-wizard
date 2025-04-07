use std::process::Command;

use anyhow::Result;

pub fn set_rdp_certificate_thumbprint(thumbprint: &str) -> Result<Vec<u8>, String> {
    // 移除指纹中可能存在的非十六进制字符
    let cleaned_thumbprint = thumbprint.replace(|c: char| !c.is_ascii_hexdigit(), "");

    // 构建 PowerShell 命令
    let ps_script = format!(
        r#"$path = (Get-WmiObject -Namespace root\cimv2\TerminalServices -Class Win32_TSGeneralSetting -Filter 'TerminalName="RDP-Tcp"').__path;
        Set-WmiInstance -Path $path -Arguments @{{ SSLCertificateSHA1Hash = "" }}
        Set-WmiInstance -Path $path -argument @{{SSLCertificateSHA1Hash="{}"}}"#,
        cleaned_thumbprint
    );

    // 执行 PowerShell 命令
    let output = Command::new("powershell")
        .args(&["-Command", &ps_script])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell command: {}", e))?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("PowerShell command failed: {}", error_msg))
    }
}
pub fn restart_rdp_service() -> Result<()> {
    let status = Command::new("powershell")
        .args(["-Command", "Restart-Service TermService -Force"])
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("RDP 服务重启失败"));
    }
    Ok(())
}