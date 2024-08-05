// modified version from https://github.com/ssankko/tonic-include-protos/blob/master/src/lib.rs
use rust_format::{Formatter, RustFmt};
use std::collections::BTreeMap;
use std::io::Write;
use tracing::debug;

#[derive(Default, Debug)]
struct ModuleStructure {
    pub mod_file: Option<String>, // for include!(mod_file.rs)
    pub children_modules: BTreeMap<String, Box<ModuleStructure>>,
}

pub fn include_protos(
    proto_folder: &str,
    source_path: &str,
    repository: &str,
    tag: &str,
) -> String {
    debug!("[{proto_folder}] Source [{source_path}] Repository[{repository}] Tag[{tag}]");
    let files = std::fs::read_dir(source_path).unwrap();
    // extract file names from output directory
    let file_names = files
        // prost constructs file names based on a grpc package name, which
        // in turn must be valid utf-8 identifier, so i use to_string_lossy fearlessly
        .map(|x| x.unwrap().file_name().to_string_lossy().to_string());

    // --------
    // traverse all files and construct tree-like structure of namespaces
    // --------
    let mut tree = ModuleStructure::default();
    for file_name in file_names {
        debug!("File Name[{}]", file_name);
        let mut current_branch = &mut tree;
        // split names by dot.
        // `tonic_build` uses dots to represent namespaces
        // for example google.logging.v2.rs will become
        // [google, logging, v2, rs]
        for part in file_name.split('.') {
            if part == "rs" {
                current_branch.mod_file = Some(file_name.to_owned());
                continue;
            } else {
                debug!("part is not rs [{part}]");
            }

            if !current_branch.children_modules.contains_key(part) {
                debug!("Addint to tree [{part}]");
                current_branch
                    .children_modules
                    .insert(part.to_owned(), Box::<ModuleStructure>::default());
            } else {
                debug!("part is on the tree already [{part}]");
            }
            current_branch = current_branch.children_modules.get_mut(part).unwrap();
        }
    }

    let out_dir = &source_path.split('/').last().unwrap();
    let mut source = Vec::new();

    // we find the "module" the root element corresponding to the proto_folder
    let root = tree
        .children_modules
        .get(proto_folder)
        .expect("Cant find module on tree");

    debug!("Root[{:#?}]", root);
    // tree to String
    write!(&mut source, "{}", construct(root, out_dir)).unwrap();

    // append meta information
    write!(
        &mut source,
        "pub mod meta {{\npub const REPOSITORY: &str = \"{}\";\npub const TAG: &str = \"{}\";\n}}",
        repository, tag
    )
    .unwrap();

    RustFmt::default()
        .format_str(String::from_utf8(source).unwrap())
        .unwrap()
}

// simple recursive function to construct mod tree based on a
// tree built earlier
fn construct(tree_entry: &ModuleStructure, out_dir: &str) -> String {
    let include = if let Some(rs_file_path) = &tree_entry.mod_file {
        format!(r#"include!("{}/{}");"#, out_dir, rs_file_path)
    } else {
        String::new()
    };
    let mods = tree_entry
        .children_modules
        .iter()
        .map(|(mod_name, module_struct)| {
            let mod_code = format!(
                "pub mod {} {{ {} }}",
                mod_name,
                construct(module_struct, out_dir)
            );
            debug!("[{mod_code}]");
            mod_code
        })
        .collect::<String>();
    format!("{include}{mods}")
}
