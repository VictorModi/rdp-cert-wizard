mod args;
mod password;
mod pfx;
mod cert_import;
mod rdp;

use std::path::PathBuf;
use args::Args;
use clap::Parser;
use anyhow::Result;
use crate::cert_import::{check_thumbprint_in_cert_store, import_pfx};

fn main() -> Result<()> {
    // 解析命令行参数
    let args = Args::parse();
    let restart = args.restart.unwrap_or_else(|| {
        true
    });

    let thumbprint = cert_import::get_cert_thumbprint(&args.cert)?;
    match check_thumbprint_in_cert_store(&*thumbprint) {
        Ok(is_exist) => {
            if !is_exist {
                let password = args.password.unwrap_or_else(|| {
                    let p = password::generate_strong_password(16);
                    println!("🔐 自动生成的 PFX 密码: {}", p);
                    p
                });
                let temp_pfx: PathBuf = pfx::save_pfx_to_temp(&args.cert, &args.key, &password)?;
                println!("save .pfx to: {}", temp_pfx.to_string_lossy());
                match import_pfx(
                    &temp_pfx, // PFX 文件路径
                    &password,                 // PFX 密码
                ) {
                    Ok(_) => println!("PFX certificate imported successfully."),
                    Err(e) => eprintln!("Error: {}", e),
                }
                pfx::cleanup_temp_pfx(&temp_pfx)?;
            }
            match rdp::set_rdp_certificate_thumbprint(&thumbprint) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Err(anyhow::anyhow!("绑定证书失败"));
                },
            };
            if restart {
                rdp::restart_rdp_service()?;
            }
            println!("✅ 证书已成功绑定到 RDP");
            Ok(())
        }
        Err(e) => {
            return Err(anyhow::anyhow!(e))
        }
    }
}
