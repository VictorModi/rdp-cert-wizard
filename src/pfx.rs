use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use crate::password;
fn get_temp_pfx_path(file_name: &str) -> PathBuf {
    let mut temp_dir = env::temp_dir();
    temp_dir.push(file_name);
    temp_dir
}

pub fn save_pfx_to_temp(cert_path: &str, key_path: &str, password: &str) -> Result<PathBuf> {
    // 生成临时文件路径
    let temp_pfx = get_temp_pfx_path(format!("{}.pfx", password::generate_strong_password(32)).as_str());

    // 使用 OpenSSL 转换 PEM 到 PFX
    let mut cmd = Command::new("openssl");
    cmd.args([
        "pkcs12", "-export", "-out", temp_pfx.to_str().unwrap(),
        "-inkey", key_path, "-in", cert_path,
        "-passout", &format!("pass:{}", password),
    ]);

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("生成 PFX 失败"));
    }

    // 返回临时文件路径
    Ok(temp_pfx)
}

pub fn cleanup_temp_pfx(pfx_path: &PathBuf) -> Result<()> {
    // 删除临时文件
    fs::remove_file(pfx_path)?;
    Ok(())
}
