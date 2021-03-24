#[cfg(feature = "base")]
fn config() -> prost_build::Config {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config
}

#[cfg(feature = "base")]
fn make_protos(protos: &[&str]) {
    tonic_build::configure()
        .compile_with_config(config(), &protos, &["."])
        .unwrap();
}

#[cfg(not(feature = "base"))]
fn make_protos(_: &[&str]) {}

fn main() {
    let mut protos = vec![];

    if cfg!(feature = "base") {
        protos.push("types/types.proto");
    }

    if cfg!(feature = "sentry") {
        protos.push("p2psentry/sentry.proto");
    }

    if cfg!(feature = "remotekv") {
        protos.push("remote/db.proto");
        protos.push("remote/ethbackend.proto");
        protos.push("remote/kv.proto");
    }

    if cfg!(feature = "snapshotsync") {
        protos.push("snapshot_downloader/external_downloader.proto");
    }

    if cfg!(feature = "txpool") {
        protos.push("txpool/txpool.proto");
        protos.push("txpool/txpool_control.proto");
    }

    make_protos(&protos);
}
