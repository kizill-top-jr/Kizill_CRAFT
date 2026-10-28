// memory_brain decides how much ram game gets and who cleans the trash
// bigger heap is a trap, longer pauses. we aint falling for it

use crate::rig_scanner::RigBiography;

pub struct GameFeedBudget {
    pub min_gb: u64,
    pub max_gb: u64, // same as min on purpose. jvm resize = stutter
    pub janitor: HeapJanitor,
    pub vibe: PlayVibe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapJanitor {
    G1Sweeper,        // default, safe, predictable pauses
    ZgcSpeedrunner,   // sub ms pauses but eats some fps. big rig only
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayVibe {
    ToasterMode,      // potato pc, 2 to 3 gb, no shaders
    SmoothCruising,   // mid rig, 4 to 6 gb, comfortable
    MovieNight,       // beefy rig, 7 to 8 gb, shaders welcome
}

// main brain. eat rig facts, spit out a budget
pub fn decide_game_food( // eta tipa mi kyshaem krch btw hihihihihi hyi
    rig: &RigBiography,
    wants_mods: bool,
    wants_shaders: bool,
) -> GameFeedBudget {
    // never eat all ram. linux and drivers need 2 gb to breathe
    let ram_reserve_gb = 2;
    let safe_ceiling = rig.ram_pantry_gb.saturating_sub(ram_reserve_gb);

    // base belly. mods eat more. shaders alone also eat a bit
    let base = match (wants_mods, wants_shaders) {
        (true, true) => 6,
        (true, false) => 4,
        (false, true) => 4,
        (false, false) => 3,
    };

    // big rigs can afford a bit more, still capped at 8
    // 16gb is a hard trap, more heap = longer pause, not more fps
    let mut belly_gb = if rig.ram_pantry_gb >= 24 && wants_mods {
        base + 2
    } else {
        base
    };

    belly_gb = belly_gb.min(8).min(safe_ceiling);

    // tiny machine? give 2 gb anyway, toaster deserves a chance
    if belly_gb < 2 {
        belly_gb = 2;
    }      // on cho ahyel kyshat stolko RAM? Dashe linux menshe jret

    // laptop with low ram? be gentle, dont cook the battery
    if rig.is_road_warrior && rig.ram_pantry_gb < 12 {
        belly_gb = belly_gb.min(4);
    }

    // zgc needs many cores and big ram, else its slower than g1
    let janitor = if rig.brain_count >= 12
        && rig.ram_pantry_gb >= 24
        && belly_gb >= 6
    {
        HeapJanitor::ZgcSpeedrunner
    } else {
        HeapJanitor::G1Sweeper
    };

    let vibe = match belly_gb {
        0..=3 => PlayVibe::ToasterMode,
        4..=6 => PlayVibe::SmoothCruising,
        _ => PlayVibe::MovieNight,
    };

    GameFeedBudget {
        min_gb: belly_gb,
        max_gb: belly_gb, // twin of min, no resizing allowed
        janitor,
        vibe,
    }
}

impl GameFeedBudget {
    // give back jvm flags. strings got dashes because java flags need em
    // we dont like dashes in prose but java aint listening to our style
    pub fn spit_jvm_flags(&self) -> String {
        let mut tokens: Vec<String> = vec![
            format!("Xms{}G", self.min_gb),
            format!("Xmx{}G", self.max_gb),
            String::from("XX:+UnlockExperimentalVMOptions"),
            String::from("XX:+DisableExplicitGC"),
            String::from("XX:+AlwaysPreTouch"),
        ];

        let janitor_bits: Vec<&str> = match self.janitor {
            HeapJanitor::G1Sweeper => vec![
                "XX:+UseG1GC",
                "XX:+ParallelRefProcEnabled",
                "XX:MaxGCPauseMillis=200",
                "XX:G1NewSizePercent=30",
                "XX:G1MaxNewSizePercent=40",
                "XX:G1HeapRegionSize=8M",
                "XX:G1ReservePercent=20",
                "XX:G1HeapWastePercent=5",
                "XX:G1MixedGCCountTarget=4",
                "XX:InitiatingHeapOccupancyPercent=15",
                "XX:G1MixedGCLiveThresholdPercent=90",
                "XX:G1RSetUpdatingPauseTimePercent=5",
                "XX:SurvivorRatio=32",
                "XX:+PerfDisableSharedMem",
                "XX:MaxTenuringThreshold=1", // na Gentoo kryche flagi
            ],
            HeapJanitor::ZgcSpeedrunner => vec![
                "XX:+UseZGC",
                "XX:+ZUncommit",
                "XX:ZUncommitDelay=300",
            ],
        };

        for bit in janitor_bits {
            tokens.push(bit.to_string());
        }

        // one dash in front. java flags start with dash. yes its annoying
        tokens
            .into_iter()
            .map(|t| format!("-{}", t))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
// **reklama or ads**
