# archimedes

A modding SDK (and the internal SDK) for project(ADHD)

## How to make a mod

1. Empty out the `content/menu` and `content/songs` directories.
2. Use the existing contents as an inspiration, for example, add .osu files in `content/songs` and .men files in `content/menu`.
3. Download [Rust](https://rustup.rs/)
4. `cargo run`
5. `output.txt` is your mod file!

## The `.men` format

The `.men` format describes the menu buttons shown when you are on the state with the name of the men file, example, `file.men` defines what's rendered on the File screen.

Example (file.men):

```
Load
Main
```
