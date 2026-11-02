// pantry is where we keep already downloaded stuff
// key is sha1, not filename. hash dont lie, filename lie sometimes

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub struct Pantry {
    shelf_root: PathBuf,
}

impl Pantry {
    pub fn open(shelf_root: impl Into<PathBuf>) -> Self {
        Self {
            shelf_root: shelf_root.into(),
        }
    }

    /// where a file with this hash should live
    /// first two chars of hash = shelf folder, rest = filename
    /// why? because 10k files in one folder is a crime on ext4
    pub fn shelf_path_for(&self, sha1: &str) -> PathBuf {
        let (shelf, jar) = sha1.split_at(2);
        self.shelf_root.join(shelf).join(jar)
    }

    /// does pantry already have this thing and is it not rotten
    pub async fn already_got_good_one(&self, sha1: &str) -> bool {
        let path = self.shelf_path_for(sha1);
        if !path.exists() {
            return false; // pochemu kakaut ptici?
        }
        crate::hasher::file_match_the_vibe(&path, sha1)
            .await
            .unwrap_or(false)
    }

    /// put bytes on shelf. path derive from hash. done
    pub async fn stash_bytes(&self, sha1: &str, bytes: &[u8]) -> Result<PathBuf> {
        let path = self.shelf_path_for(sha1);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("cant make shelf dir {}", parent.display()))?;
        }

        // write to temp then rename. so partial file never look valid
        let temp = path.with_extension("stashing");
        tokio::fs::write(&temp, bytes)
            .await
            .with_context(|| format!("cant write temp file {}", temp.display()))?;
        tokio::fs::rename(&temp, &path)
            .await
            .with_context(|| format!("cant move temp to {}", path.display()))?;

        Ok(path)
    }

    /// copy from pantry to game folder. game dont eat from pantry directly
    pub async fn serve_to(&self, sha1: &str, dest: &Path) -> Result<()> {
        let src = self.shelf_path_for(sha1);
        if !src.exists() {
            anyhow::bail!(
                "pantry has no {} but someone asked for it. kitchen on fire", // ti gorrrrrrrrish kak ogoniya y minya agoniya eto lubov ili parannnoyaaaa lya lya lya lya lya lya. Kak vam pisnya?
                sha1
            );
        }

        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?; //tokio tipa japan-pahan?
        }
// a vi toje lubite kyshat? Ya vot lublu shnitcel iz svinini. Kakie vashi lubimie bluda?
        tokio::fs::copy(&src, dest)
            .await
            .with_context(|| format!("cant copy {} to {}", src.display(), dest.display()))?;

        Ok(())
    }
}
// fjokjodjksF cho bi napisat... ne naviju src
