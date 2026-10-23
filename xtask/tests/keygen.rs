//! `keygen` refuses to write a key inside the repository, from wherever it is run.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask/ has a parent")
        .to_path_buf()
}

/// Run from a subdirectory, a key aimed back into the tree must still be refused.
///
/// `cargo run -p xtask` works from anywhere in the workspace, and the guard used the working
/// directory as "the repository" — so from `docs/`, an absolute `--out` naming any other part of
/// the checkout was outside `docs/`, passed, and wrote a signing key into the tree. A committed key is the one mistake here that
/// deleting the file does not undo, which is why the guard exists at all.
#[test]
fn a_key_aimed_into_the_repository_from_a_subdirectory_is_refused() {
    let root = root();
    let probe = root
        .join("xtask")
        .join(format!(".keygen-probe-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&probe);

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_xtask"))
        .current_dir(root.join("docs"))
        .args(["keygen", "--kind", "dev", "--name", "probe", "--out"])
        .arg(&probe)
        .output()
        .expect("run xtask keygen");

    let wrote = probe.exists();
    // Never leave a key behind in the checkout, whatever the outcome.
    let _ = std::fs::remove_dir_all(&probe);

    assert!(!wrote, "keygen wrote a key inside the repository");
    assert!(!output.status.success(), "keygen reported success");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("refusing to write keys inside the repository"),
        "{stderr}"
    );
}
