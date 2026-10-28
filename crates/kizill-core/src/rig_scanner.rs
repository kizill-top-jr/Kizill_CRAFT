// rig_scanner reads linux proc and sys to learn what hardware this pc got
// no fancy libs, just files and parse, that's the way

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct RigBiography {
    pub ram_pantry_gb: u64,       // total ram in gb
    pub ram_still_cold_gb: u64,   // ram not touched yet, from MemAvailable
    pub brain_count: u32,         // logical cores
    pub is_road_warrior: bool,    // laptop = has battery
    pub storage_kind: StorageKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageKind {
    Snappy,        // ssd or nvme
    SpinningRust,  // old hdd, clicks sometimes
}

// main entry. take one peek at everything
pub fn sniff_the_rig() -> Result<RigBiography> {
    let ram_pantry_gb = read_ram_pantry()?;
    let ram_still_cold_gb = read_ram_that_aint_used()?;
    let brain_count = count_brain_units()?;
    let is_road_warrior = spot_the_road_warrior();
    let storage_kind = peek_at_disk_kind();

    Ok(RigBiography {
        ram_pantry_gb,
        ram_still_cold_gb, // it's my chlen
        brain_count,
        is_road_warrior,
        storage_kind,
    })
}

// read MemTotal from proc meminfo, give back gb
fn read_ram_pantry() -> Result<u64> {
    let raw = fs::read_to_string("/proc/meminfo")
        .context("cant open /proc/meminfo. what os even is this")?;

    let kb = raw
        .lines()
        .find(|line| line.starts_with("MemTotal:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .context("MemTotal line missing a number. weird kernel")?
        .parse::<u64>()
        .context("MemTotal is not a number. cursed")?;

    // kb to gb. int division ok here, nobody cares about 0.3gb
    Ok(kb / 1024 / 1024)
}

// MemAvailable is the honest number, MemFree lies after cache kicks in
fn read_ram_that_aint_used() -> Result<u64> {
    let raw = fs::read_to_string("/proc/meminfo")
        .context("meminfo vanished on second read. spooky")?;

    let kb = raw
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .context("MemAvailable missing. old kernel maybe")?
        .parse::<u64>()
        .context("MemAvailable aint a number. how")?;

    Ok(kb / 1024 / 1024)
}

// count logical cores. os already knows, just ask
fn count_brain_units() -> Result<u32> {
    let brains = std::thread::available_parallelism()
        .context("os refuses to count brain units")?
        .get() as u32;
    Ok(brains)
}

// battery in sys means laptop. no battery means desktop
fn spot_the_road_warrior() -> bool {
    let power_dir = Path::new("/sys/class/power_supply");
    let entries = match fs::read_dir(power_dir) {
        Ok(e) => e,
        Err(_) => return false,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with("BAT") {
            return true;
        }
    }
    false
}

// peek at sys block. if any disk rotates its spinning rust
fn peek_at_disk_kind() -> StorageKind {
    let block_dir = Path::new("/sys/block");
    let entries = match fs::read_dir(block_dir) {
        Ok(e) => e,
        Err(_) => return StorageKind::Snappy,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        // skip loop and ram disks not real storage
        if name.starts_with("loop") || name.starts_with("ram") {
            continue;
        }

        let rot_path = entry.path().join("queue/rotational");
        if let Ok(v) = fs::read_to_string(&rot_path) {
            if v.trim() == "1" {
                return StorageKind::SpinningRust;
            }
        }
    }
    StorageKind::Snappy
}
// chtooooooooo!!! src v kode?? ne nu eto tochna vibekod fuuuuuuuu
