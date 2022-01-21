pub fn greet(script_path: &std::path::Path, name: &str) -> std::io::Result<std::process::ExitStatus> {
    std::process::Command::new(script_path)
        .arg(name)
        .spawn()?
        .wait()
}

pub fn main() -> std::io::Result<()> {
    let script_path = std::env::temp_dir().join("hello.cmd");
    std::fs::write(&script_path, "@echo Hello, %~1!")?;
    greet(&script_path, "world")?;
    greet(&script_path, "fellow Rustaceans")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn vbs() -> std::io::Result<()> {
        let script_path = std::env::temp_dir().join("hello.vbs");
        std::fs::write(&script_path, r#"WScript.Echo("Hello, World!")"#)?;
        greet(&script_path, "world")?;
        greet(&script_path, "fellow Rustaceans")?;
        Ok(())
    }
}
