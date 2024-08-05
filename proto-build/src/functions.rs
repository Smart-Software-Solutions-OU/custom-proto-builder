use std::{
    fs::{copy, create_dir_all, remove_dir_all, remove_file},
    path::{Path, PathBuf},
};

use tracing::{debug, error, info};
use walkdir::WalkDir;

/// Copy generated files to target folder
pub fn copy_files(
    src_dir: &Path,
    target_dir: &Path,
    proto_folder: &str,
    target_mod: &Path,
    merge: bool,
) {
    // we use merge for specific ics23 since its expected on cosmos module....
    if !merge {
        // Remove old compiled files
        remove_dir_all(target_dir).unwrap_or_default();
        remove_file(target_mod).unwrap_or_default();
    }
    create_dir_all(target_dir).unwrap();

    info!("Working on [{:#?}] using [{}]", src_dir, proto_folder);
    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(src_dir)
        .contents_first(true)
        .into_iter()
        .filter_entry(|e| {
            debug!("DirEntry[{:#?}]", e);
            let is_file = e.file_type().is_file();
            // let is_in_proto_folder = e
            //     .file_name()
            //     .to_str()
            //     .map(|name| name.starts_with(format!("{}.", proto_folder).to_string().as_str()))
            //     .unwrap_or(false);
            let is_in_proto_folder = e
                .file_name()
                .to_str()
                .map(|name| name.ends_with(".rs"))
                .unwrap_or(false);
            debug!(
                "is_file[{}] is_in_proto_folder[{}]",
                is_file, is_in_proto_folder
            );
            is_file && is_in_proto_folder
        })
        .map(|res| {
            let e = res?;
            let from = e.path();
            let to = target_dir.join(e.file_name());
            debug!("Copying [{:#?}] -> [{:#?}]", from, to);
            copy(from, to)
        })
        .filter_map(|res| res.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            error!("Error while copying compiled file: {e}");
        }
        panic!("[error] => Aborted.");
    }
}

/// Walk through the directory recursively and gather all *.proto files
pub fn find_proto_files(
    proto_path: &Path,
    allowed_modules: Vec<String>,
    black_list: Vec<String>,
) -> Vec<PathBuf> {
    let protos: Vec<PathBuf> = WalkDir::new(proto_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            // Check if it's a directory and its name is in allowed modules
            allowed_modules
                .iter()
                .any(|e| entry.path().to_string_lossy().to_string().contains(e))
        })
        .filter(|entry| {
            // apply blacklist
            if !black_list.is_empty() {
                black_list
                    .iter()
                    .any(|e| !entry.path().to_string_lossy().to_string().contains(e))
            } else {
                true
            }
        })
        .filter(|entry| {
            entry.file_type().is_file()
                && entry.path().extension().is_some()
                && entry.path().extension().unwrap() == "proto"
        })
        // case staking and authz have Validators struct and enum with the same name generated
        .filter(|entry| {
            // if !(entry
            //     .path()
            //     .to_string_lossy()
            //     .to_string()
            //     .contains("staking"))
            // {
            //     warn!("{entry:#?}");
            // }
            !(entry.file_type().is_file()
                && entry
                    .path()
                    .to_string_lossy()
                    .to_string()
                    .contains("staking")
                && entry.path().to_string_lossy().to_string().contains("authz"))
        })
        .map(|e| e.into_path())
        .collect();
    protos
}
