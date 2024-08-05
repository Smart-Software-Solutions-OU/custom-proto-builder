mod buf_build;
mod config;
mod functions;
mod git;
mod mod_builder;
mod proto;

use std::io::Write;
use std::{
    env::var,
    fs::{self, File},
    path::PathBuf,
    process,
};

use tempfile::tempdir;
use tracing::{error, info};
use tracing_subscriber::{
    filter, fmt,
    prelude::*,
    reload::{self},
};

use crate::{
    buf_build::{export_dep_module, read_locked_deps},
    config::ProjectConfig,
    git::get_commitish,
    mod_builder::include_protos,
    proto::{get_current_working_dir, ProtoConfig},
};

use functions::{copy_files, find_proto_files};

pub const CONFIG_PATH: &str = "./config";

pub const GENERATE_MOD: bool = true;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = filter::LevelFilter::INFO;
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO)?;
    info!("Path {}", get_current_working_dir());

    let config = ProjectConfig::from_yaml(CONFIG_PATH);

    for (name, project) in config.projects.iter() {
        info!("Processing {} project.", name);

        let proto_config = ProtoConfig::from_yaml(&format!("{}/{}", CONFIG_PATH, project.path));
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let target_dir = project.target_dir.iter().collect::<PathBuf>();

        for proto in proto_config.protos {
            let proto_folder = proto.proto_folder.as_str();
            let ver_target_dir = target_dir.join(proto_folder);
            let target_mod = target_dir.join(format!("{}.rs", proto_folder));

            // check if we use github or local file
            let (proto_github_path, proto_includes_paths, temp_dirs) =
                if proto.repository.starts_with("http") {
                    let repo_dir = proto.repository.split('/').last().unwrap();
                    let github_dir = PathBuf::from(
                        root.join("..")
                            .join("target")
                            .join(repo_dir)
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );

                    let proto_github_path = github_dir.join("proto");

                    // git fetch proto.repository url into target directory
                    info!(
                        "Fetching {} at {} into {github_dir:?}",
                        proto.repository, proto.tag,
                    );
                    get_commitish(&github_dir, &proto.repository, &proto.tag); // This panics if it fails.

                    let mut proto_includes_paths = vec![proto_github_path.clone()];
                    let buf_lock_path = proto_github_path.join("buf.lock");

                    // This lives on the scope
                    let temp_dirs = match read_locked_deps(&buf_lock_path, &proto_github_path) {
                        Ok(deps) => deps
                            .iter()
                            .map(|dep| {
                                let mod_dir = tempdir().unwrap();
                                if let Err(e) = export_dep_module(dep, mod_dir.path()) {
                                    error!(
                                        "Failed to export module {}/{}/{}: {}",
                                        dep.remote, dep.owner, dep.repository, e,
                                    );
                                    process::exit(1);
                                }
                                proto_includes_paths.push(mod_dir.path().to_owned());
                                mod_dir
                            })
                            .collect::<Vec<_>>(),
                        Err(e) => {
                            error!("Failed to read {}: {}", buf_lock_path.display(), e);
                            process::exit(1);
                        }
                    };

                    info!("OK");
                    (proto_github_path, proto_includes_paths, temp_dirs)
                } else {
                    // for now, local dont use buf.yaml, nor the generated buf.lock that include the deps
                    // https://vchitai.medium.com/using-buf-build-to-generate-your-grpc-codes-44e1811d5291
                    let proto_github_path = PathBuf::from(
                        root.join("..")
                            .join(&proto.repository)
                            .join(&proto.proto_folder)
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                    let proto_includes_paths = vec![proto_github_path.clone()];
                    (
                        proto_github_path,
                        proto_includes_paths,
                        vec![tempdir().unwrap()],
                    )
                };
            // List available proto files

            let protos = find_proto_files(&proto_github_path, proto.modules, proto.black_list);
            let out_dir = var("OUT_DIR")
                .map(PathBuf::from)
                .or_else(|_| tempdir().map(|d| d.into_path()))
                .unwrap();
            let descriptor_file = out_dir.clone().join("descriptors.bin");
            // Configure prost builder
            let mut pb = prost_build::Config::new();

            // Enable generation of `prost::Name` annotations for all types
            pb.enable_type_names();

            // Apply custom attributes from config.yaml for every proto
            for (path, attr) in proto.custom {
                pb.type_attribute(path, attr);
            }

            // Default mapping to avoid dependencies problems.
            // pb.extern_path(
            //     ".google.protobuf.BytesValue",
            //     "::prost::alloc::vec::Vec<u8>",
            // );
            // pb.extern_path(
            //     ".google.protobuf.StringValue",
            //     "::prost::alloc::string::String",
            // );
            // .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
            // .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
            // .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any");

            // Tonic builder configuration for cliente and transport
            // we use well know types and prost_wkt_types to avoid google dependencies and others
            let client_mod_attribute = format!("#[cfg(feature = \"{}\")]", proto.proto_folder);
            let server_mod_attribute = format!("#[cfg(feature = \"{}\")]", proto.proto_folder);
            let builder = tonic_build::configure()
                .out_dir(&out_dir)
                .build_server(true)
                .build_client(true)
                .build_transport(true)
                .client_mod_attribute(".", client_mod_attribute)
                .server_mod_attribute(".", server_mod_attribute)
                //.compile_well_known_types(true)
                .file_descriptor_set_path(&descriptor_file);

            // Build
            info!("Building structs and interfaces.");
            info!("Protos [{:#?}]", protos);
            info!("Include Paths [{:#?}]", proto_includes_paths);
            info!("Temp dirs [{:#?}]", temp_dirs);

            match builder.compile_with_config(pb, &protos, &proto_includes_paths) {
                Ok(()) => {
                    info!("Proto files successfully processed.")
                }
                Err(e) => {
                    error!("{}", e);
                    process::exit(1);
                }
            }
            //let descriptor_bytes = std::fs::read(descriptor_file).unwrap();

            //let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

            //prost_wkt_build::add_serde(out_dir.clone(), descriptor);

            // move files to destination.
            info!(
                "Removing old structs and copying new structs to {}",
                ver_target_dir.to_string_lossy(),
            );
            fs::create_dir_all(&target_dir).unwrap();

            copy_files(
                &out_dir,
                &ver_target_dir,
                proto_folder,
                &target_mod,
                proto.merge,
            );

            // generate mod file with includes.
            info!(
                "Generating mod file for {}",
                target_mod.to_string_lossy().to_string()
            );
            let include_protos = include_protos(
                proto_folder,
                &ver_target_dir.to_string_lossy(),
                &proto.repository,
                &proto.tag,
            );

            let mut file = File::create(target_mod).unwrap();
            writeln!(&mut file, "{}", include_protos).unwrap();

            info!("Done!");
        }
    }
    Ok(())
}
