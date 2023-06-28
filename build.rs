use std::{env, path::PathBuf};

fn config() -> prost_build::Config {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config
}

fn make_protos(protos: &[&str]) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile_with_config(config(), protos, &["."])
        .unwrap();
}

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let mut protos = vec!["types/types.proto"];

    if cfg!(feature = "sentry") {
        protos.push("p2psentry/sentry.proto");
    }

    if cfg!(feature = "sentinel") {
        protos.push("p2psentinel/sentinel.proto");
    }

    if cfg!(feature = "remotekv") {
        protos.push("remote/ethbackend.proto");
        protos.push("remote/kv.proto");
    }

    if cfg!(feature = "snapshotsync") {
        protos.push("downloader/downloader.proto");
    }

    if cfg!(feature = "txpool") {
        protos.push("txpool/mining.proto");
        protos.push("txpool/txpool.proto");
    }

    if cfg!(feature = "execution") {
        protos.push("execution/execution.proto");
    }

    if cfg!(feature = "engine") {
        protos.push("engine/engine.proto");
    }

    if cfg!(feature = "web3") {
        protos.push("web3/common.proto");
        protos.push("web3/debug.proto");
        protos.push("web3/eth.proto");
        protos.push("web3/trace.proto");
    }

    make_protos(&protos);
}
