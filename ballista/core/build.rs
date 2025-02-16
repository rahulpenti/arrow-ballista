// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

fn main() -> Result<(), String> {
    use std::io::Write;

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    println!("cargo:rerun-if-changed=proto/ballista.proto");
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
    println!("cargo:rerun-if-changed=proto/datafusion.proto");
    tonic_build::configure()
        .extern_path(".datafusion", "::datafusion_proto::protobuf")
        .compile(&["proto/ballista.proto"], &["proto"])
        .map_err(|e| format!("protobuf compilation failed: {}", e))?;

    // TODO: undo when resolved: https://github.com/intellij-rust/intellij-rust/issues/9402
    #[cfg(feature = "docsrs")]
    let path = out.join("ballista.rs");
    #[cfg(not(feature = "docsrs"))]
    let path = "src/serde/generated/ballista.rs";

    let code = std::fs::read_to_string(out.join("ballista.protobuf.rs")).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    file.write_all(code.as_str().as_ref()).unwrap();

    Ok(())
}
