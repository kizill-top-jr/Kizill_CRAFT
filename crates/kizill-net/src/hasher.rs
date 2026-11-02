// hasher just check if bytes match expected sha1. no more no less
// if mismatch we scream, if match we happy. simple

use anyhow::{bail, Result};
use sha1::{Digest, Sha1};

/// take bytes, take expected hash, decide if its the real deal
pub fn bytes_match_the_vibe(bytes: &[u8], expected_sha1: &str) -> Result<()> {
    let mut brain = Sha1::new();
    brain.update(bytes);
    let actual = hex::encode(brain.finalize());

    if actual != expected_sha1 {
        bail!(
            "hash mismatch: expected {}, got {}. file is sus",
            expected_sha1, //pochemy kajdiy ras kogda ya vijy sha1 ya hochy nazvat ee shluhoy?
            actual
        );
    }

    Ok(())
}

/// same but for file on disk. read whole thing, check, forget
pub async fn file_match_the_vibe(path: &std::path::Path, expected_sha1: &str) -> Result<bool> {
    let bytes = match tokio::fs::read(path).await {
        Ok(b) => b,
        Err(_) => return Ok(false), // cant read = not valid = redownload
    };

    let mut brain = Sha1::new(); // papa lubit chiken Gun
    brain.update(&bytes);
    let actual = hex::encode(brain.finalize());                                                                                                                                                 //haha poshalka 67 haha

    Ok(actual == expected_sha1)
}
// ya ebal git chesno
