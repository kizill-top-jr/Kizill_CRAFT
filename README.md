# Kizill_CRAFT

Minecraft launcher built for people who are tired of java stutter

Rust core, native linux, smart ram budget, no vibe code inside

## what it is

Kizill_CRAFT is a Minecraft launcher that actually thinks about performance
Not another electron wrapper around the official launcher
Not a GUI with buttons that do nothing

It reads your hardware, decides how much memory the game deserves, picks the right garbage collector, and gets out of the way

## why

Minecraft on java has a known problem
Game allocates thousands of short lived objects per tick
Heap fills up, garbage collector kicks in, main thread pauses
Player feels a micro freeze even on a beefy machine

Most launchers solve this by giving the game 16gb of ram
This makes the pauses longer, not shorter
Because garbage collector has to scan more memory in one go

Kizill_CRAFT solves it differently
Caps heap at 8gb no matter what
Sets Xms equal to Xmx so jvm never resizes
Picks G1GC with aikar style flags by default
Offers ZGC only for rigs that actually benefit from it
Feeds MemorySweep and FerriteCore into modded profiles

## features

working right now

- linux hardware scanner that reads `/proc` and `/sys` directly
- heap budget brain that caps at 8gb and never lies about it
- sha1 verified download pipeline with hash keyed cache
- async downloader with concurrency pool

in the works

- version manifest parser
- library resolver with native library handling for linux
- asset index and object store
- jvm argument builder with classpath assembly
- jre detection and management
- process launcher with proper environment setup
- GUI on top of working engine

## planned

- mod profile management
- shader pack installation
- performance benchmark comparing vanilla vs kizill build

## build

needs rust 1.75 or newer, any linux distro

```bash
git clone git@github.com:kizill-top-jr/Kizill_CRAFT.git
cd Kizill_CRAFT
cargo build --release
```
## example run

```bash

cargo run --example test_grab -p kizill-net
```

## structure

```text

kizill-launcher/
├── crates/
│   ├── kizill-core/     hardware scanner, heap budget brain
│   ├── kizill-net/      downloader, sha1 hasher, hash keyed pantry
│   ├── kizill-patch/    version resolution, jvm args, mods (planned)
│   └── kizill-native/   process launch, linux env setup (planned)
├── data/                local cache, not tracked
└── docs/                architecture notes (planned)
```
versioning

This project uses four number tags: 0.0.0.1, 0.0.0.2, and so on
Not semver, just how we do it here

## license

GNU General Public License v3.0
See [LICENSE](https://github.com/kizill-top-jr/Kizill_CRAFT?tab=GPL-3.0-1-ov-file) file for full text

##**privet**##
