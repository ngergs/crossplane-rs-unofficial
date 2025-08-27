use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind::NotFound;
use std::io::{Error, Write};
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TypeMetaCatchAll {
    pub api_version: String,
    pub kind: String,
    #[serde(flatten)]
    other: HashMap<String, serde_yaml_ng::Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("src/generated")?;
    let xrd_raw = fs::read_to_string("schema/xrd.yaml")?;
    let mut xrd = serde_yaml_ng::from_str::<TypeMetaCatchAll>(&xrd_raw)?;
    // kopium only supports native k8s crd's but all relevant fields are the same for crossplane xrd's
    // so we can just adjust the apiVersion and Kind
    "apiextensions.k8s.io/v1".clone_into(&mut xrd.api_version);
    "CustomResourceDefinition".clone_into(&mut xrd.kind);
    let xrd_raw = serde_yaml_ng::to_string(&xrd)?;

    let kopium_binary_name = "kopium";
    let mut kopium_cmd = Command::new(kopium_binary_name)
        .args(["--filename", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| if e.kind() == NotFound {
            Error::new(NotFound,format!("Binary `{kopium_binary_name}` not available in the PATH. This is a compile-time dependency (not needed at runtime)."))
        } else { e })?;
    kopium_cmd
        .stdin
        .as_mut()
        .ok_or("kopium build subcommands stdin is not available")?
        .write_all(xrd_raw.as_bytes())?;
    let kopium_output = kopium_cmd.wait_with_output()?;
    if !kopium_output.status.success() {
        return Err(format!(
            "kopium build failed {}:",
            String::from_utf8(kopium_output.stderr)?
        )
        .into());
    }

    fs::write("src/generated/xrd.rs", kopium_output.stdout)?;
    Ok(())
}
