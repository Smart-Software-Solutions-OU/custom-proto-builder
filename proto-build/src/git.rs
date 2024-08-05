use std::path::Path;

use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    AutotagOption, Commit, FetchOptions, Oid, Reference, Repository,
};
use subtle_encoding::hex;
use tracing::debug;

/// Clone or open+fetch a repository and check out a specific commitish
/// In case of an existing repository, the origin remote will be set to `url`.
pub fn get_commitish(dir: &Path, url: &str, commitish: &str) {
    let repo = if dir.exists() {
        fetch_existing(dir, url)
    } else {
        clone_new(dir, url)
    };
    checkout_commitish(&repo, commitish)
}

fn clone_new(dir: &Path, url: &str) -> Repository {
    debug!(
        "  [info] => Cloning {} into {} folder",
        url,
        dir.to_string_lossy()
    );

    let mut fo = FetchOptions::new();
    fo.download_tags(AutotagOption::All);
    fo.update_fetchhead(true);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    builder.clone(url, dir).unwrap()
}

fn fetch_existing(dir: &Path, url: &str) -> Repository {
    debug!(
        "  [info] => Fetching from {} into existing {} folder",
        url,
        dir.to_string_lossy()
    );
    let repo = Repository::open(dir).unwrap();

    let mut fo = git2::FetchOptions::new();
    fo.download_tags(git2::AutotagOption::All);
    fo.update_fetchhead(true);

    let mut remote = repo
        .find_remote("origin")
        .unwrap_or_else(|_| repo.remote("origin", url).unwrap());
    if remote.url().is_none() || remote.url().unwrap() != url {
        repo.remote_set_url("origin", url).unwrap();
    }
    debug!("  [info] => Fetching repo using remote `origin`");
    let specs: &[&str] = &[];
    remote.fetch(specs, Some(&mut fo), None).unwrap();

    let stats = remote.stats();
    if stats.local_objects() > 0 {
        debug!(
            "  [info] => Received {}/{} objects in {} bytes (used {} local objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        debug!(
            "  [info] => Received {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    Repository::open(dir).unwrap()
}

fn checkout_commitish(repo: &Repository, commitish: &str) {
    let (reference, commit) = find_reference_or_commit(repo, commitish);

    debug!(
        "  [info] => Checking out repo in detached HEAD mode:\n    \
             [info] => id: {},\n    \
             [info] => author: {},\n    \
             [info] => committer: {},\n    \
             [info] => summary: {}",
        commit.id(),
        commit.author(),
        commit.committer(),
        commit.summary().unwrap_or(""),
    );

    match reference {
        None => repo.set_head_detached(commit.id()).unwrap(),
        Some(reference) => {
            debug!("    [info] => name: {}", reference.shorthand().unwrap());
            repo.set_head(reference.name().unwrap()).unwrap();
        }
    }

    let mut checkout_options = CheckoutBuilder::new();
    checkout_options
        .force()
        .remove_untracked(true)
        .remove_ignored(true)
        .use_theirs(true);
    repo.checkout_head(Some(&mut checkout_options)).unwrap();
}

fn find_reference_or_commit<'a>(
    repo: &'a Repository,
    commitish: &str,
) -> (Option<Reference<'a>>, Commit<'a>) {
    let mut tried_origin = false; // we tried adding 'origin/' to the commitish

    let mut try_reference = repo.resolve_reference_from_short_name(commitish);
    if try_reference.is_err() {
        // Local branch might be missing, try the remote branch
        try_reference = repo.resolve_reference_from_short_name(&format!("origin/{commitish}"));
        tried_origin = true;
        if try_reference.is_err() {
            // Remote branch not found, last chance: try as a commit ID
            // Note: Oid::from_str() currently does an incorrect conversion and cuts the second half
            // of the ID. We are falling back on Oid::from_bytes() for now.
            let commitish_vec =
                hex::decode(commitish).unwrap_or_else(|_| hex::decode_upper(commitish).unwrap());
            return (
                None,
                repo.find_commit(Oid::from_bytes(commitish_vec.as_slice()).unwrap())
                    .unwrap(),
            );
        }
    }

    let mut reference = try_reference.unwrap();
    if reference.is_branch() {
        if tried_origin {
            panic!("[error] => local branch names with 'origin/' prefix not supported");
        }
        try_reference = repo.resolve_reference_from_short_name(&format!("origin/{commitish}"));
        reference = try_reference.unwrap();
        if reference.is_branch() {
            panic!("[error] => local branch names with 'origin/' prefix not supported");
        }
    }

    let commit = reference.peel_to_commit().unwrap();
    (Some(reference), commit)
}
