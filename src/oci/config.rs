use std::error;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::value::Value;
use std::collections::HashMap;

use crate::oci::Hash;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OS {
    Linux,
    Windows,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Architecture {
    Amd64,
    Aarch64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RootFSType {
    Layers,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Configuration {
    user: String,
    exposed_ports: HashMap<String, Value>,
    env: Vec<String>,
    entrypoint: Vec<String>,
    cmd: Vec<String>,
    volumes: HashMap<String, Value>,
    working_dir: String,
    labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootFS {
    diff_ids: Vec<Hash>,
    #[serde(alias = "type")]
    rootfs_type: RootFSType,
}

fn default_as_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    created: String,
    created_by: String,
    #[serde(default = "default_as_false")]
    empty_layer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    created: String,
    author: String,
    architecture: Architecture,
    os: OS,
    config: Configuration,
    rootfs: RootFS,
    history: Vec<History>,
}

pub fn parse_config(content: &str) -> Result<Config, Box<dyn error::Error>> {
    let c = serde_json::from_str(content)?;
    Ok(c)
}

#[test]
fn test_parse_config_ok() {
    let c = r#"
        {
            "created": "2015-10-31T22:22:56.015925234Z",
            "author": "Alyssa P. Hacker <alyspdev@example.com>",
            "architecture": "amd64",
            "os": "linux",
            "config": {
                "User": "alice",
                "ExposedPorts": {
                    "8080/tcp": {}
                },
                "Env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "FOO=oci_is_a",
                    "BAR=well_written_spec"
                ],
                "Entrypoint": [
                    "/bin/my-app-binary"
                ],
                "Cmd": [
                    "--foreground",
                    "--config",
                    "/etc/my-app.d/default.cfg"
                ],
                "Volumes": {
                    "/var/job-result-data": {},
                    "/var/log/my-app-logs": {}
                },
                "WorkingDir": "/home/alice",
                "Labels": {
                    "com.example.project.git.url": "https://example.com/project.git",
                    "com.example.project.git.commit": "45a939b2999782a3f005621a8d0f29aa387e1d6b"
                }
            },
            "rootfs": {
              "diff_ids": [
                "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
                "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef"
              ],
              "type": "layers"
            },
            "history": [
              {
                "created": "2015-10-31T22:22:54.690851953Z",
                "created_by": "/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /"
              },
              {
                "created": "2015-10-31T22:22:55.613815829Z",
                "created_by": "/bin/sh -c #(nop) CMD [\"sh\"]",
                "empty_layer": true
              }
            ]
        }"#;
    let m = parse_config(c);

    if m.is_err() {
        println!("Error {:?}", m.err());
        assert!(false);
    } else {
        assert!(m.is_ok());
        println!("Result: {:?}", m.unwrap());
    }
}
