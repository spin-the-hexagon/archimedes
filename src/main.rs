use crate::error::ADHDError;
use crate::script::{ContentScriptOpcode, ScriptWriter};
use eyre::{Report, Result};
use rayon::prelude::*;
use rosu_map::section::hit_objects::HitObjectKind;
use rosu_map::util::StrExt;
use rosu_map::Beatmap;
use std::fs::{read_dir, read_to_string, write, DirEntry};
use std::io::Error;

pub mod error;
pub mod lists;
pub mod script;

fn main() -> Result<()> {
    let songs: Result<Vec<_>> = read_dir("./content/songs")?
        .collect::<Result<Vec<DirEntry>, Error>>()?
        .par_iter()
        .map(load_song)
        .collect();

    let menus: Result<Vec<_>> = read_dir("./content/menu")?
        .collect::<Result<Vec<DirEntry>, Error>>()?
        .par_iter()
        .map(load_menu)
        .collect();

    let songs = songs?;

    println!("loaded {} songs", songs.len());

    let mut output = ScriptWriter::new();

    for Song { name, script } in songs {
        output.emit(ContentScriptOpcode::BeatmapCollectionRegister { name, script })
    }

    for Menu { lines, state } in menus? {
        for (pos, line) in lines.iter().enumerate() {
            output.emit(ContentScriptOpcode::AddMenuItem {
                pos: pos as i32,
                next_state: line.clone(),
                state: state.clone(),
            })
        }
    }

    write("./output.txt", output.text()?)?;

    Ok(())
}

struct Song {
    name: String,
    script: String,
}

struct Menu {
    lines: Vec<String>,
    state: String,
}

fn load_song(file: &DirEntry) -> Result<Song> {
    match file.path().extension() {
        Some(x) => match x.to_string_lossy().as_ref() {
            "osu" => load_osu(file),
            "v2" => load_legacy_v2(file),
            _ => Err(Report::from(ADHDError::InvalidFileExtension(
                file.path().display().to_string(),
            ))),
        },
        _ => Err(Report::from(ADHDError::InvalidFileExtension(
            file.path().display().to_string(),
        ))),
    }
}

fn load_menu(file: &DirEntry) -> Result<Menu> {
    match file.path().extension() {
        Some(x) => match x.to_string_lossy().as_ref() {
            "men" => {
                let text = read_to_string(file.path())?;

                let lines = text.lines().collect::<Vec<_>>();

                Ok(Menu {
                    lines: lines.iter().map(|x| x.to_string()).collect(),
                    state: file.path().file_stem().unwrap().display().to_string(),
                })
            }
            _ => Err(Report::from(ADHDError::InvalidFileExtension(
                file.path().display().to_string(),
            ))),
        },
        _ => Err(Report::from(ADHDError::InvalidFileExtension(
            file.path().display().to_string(),
        ))),
    }
}

fn load_osu(file: &DirEntry) -> Result<Song> {
    let text = read_to_string(file.path())?;

    let beatmap = rosu_map::from_str::<Beatmap>(&text)?;

    let mut writer = ScriptWriter::new();

    for object in beatmap.hit_objects {
        match object.kind {
            HitObjectKind::Circle(circle) => {
                let playfield_width = 512;
                let columns = 4;
                let column_width = playfield_width / columns;
                let column = (circle.pos.x / column_width as f32) as i32;

                writer.emit(ContentScriptOpcode::BeatmapNote {
                    time: object.start_time / 1000.0,
                    lane: column + 1,
                });
            }
            HitObjectKind::Hold(hold) => {
                let playfield_width = 512;
                let columns = 4;
                let column_width = playfield_width / columns;
                let column = (hold.pos_x / column_width as f32) as i32;

                writer.emit(ContentScriptOpcode::BeatmapNote {
                    time: object.start_time / 1000.0,
                    lane: column + 1,
                    //duration: hold.duration / 1000.0
                });

                writer.emit(ContentScriptOpcode::BeatmapNote {
                    time: (object.start_time + hold.duration) / 1000.0,
                    lane: column + 1,
                });
            }
            other => todo!("{:#?}", other),
        }
    }

    Ok(Song {
        name: file.path().file_stem().unwrap().display().to_string(),
        script: writer.text()?,
    })
}

fn load_legacy_v2(file: &DirEntry) -> Result<Song> {
    let text = read_to_string(file.path())?;

    let mut writer = ScriptWriter::new();

    let mut segments: Vec<_> = text.split("X").collect();

    // remove magic
    assert_eq!(segments.remove(0), "V2");

    // Remove last empty segment
    assert_eq!(segments.pop(), Some(""));

    // Parse
    segments.reverse();

    while let Some(opcode) = segments.pop() {
        match opcode {
            "O" => {
                let time = segments
                    .pop()
                    .map(|x| x.parse_num::<f64>().ok())
                    .flatten()
                    .unwrap();
                let lane = segments
                    .pop()
                    .map(|x| x.parse_num::<i32>().ok())
                    .flatten()
                    .unwrap();

                writer.emit(ContentScriptOpcode::BeatmapNote { time, lane })
            }
            other => todo!("unknown v2 opcode {other}"),
        }
    }

    Ok(Song {
        name: file.path().file_stem().unwrap().display().to_string(),
        script: writer.text()?,
    })
}
