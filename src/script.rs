use crate::lists::vec_to_string;
use crate::script::ContentScriptOpcode::{
    AddMenuItem, BeatmapCollectionRegister, BeatmapModifier, BeatmapNote, OpcodeRegister,
};
use eyre::Result;

pub enum ContentScriptOpcode {
    OpcodeRegister {
        id: String,
    },
    BeatmapNote {
        time: f64,
        lane: i32,
    },
    BeatmapModifier {
        from: f64,
        to: f64,
        modifier: String,
    },
    BeatmapCollectionRegister {
        name: String,
        script: String,
    },
    AddMenuItem {
        pos: i32,
        next_state: String,
        state: String,
    },
}

pub struct ScriptWriter {
    segments: Vec<String>,
    opcodes: Vec<String>,
}

impl ScriptWriter {
    pub fn new() -> Self {
        Self {
            segments: vec![],
            opcodes: vec!["opcode_register".to_string()],
        }
    }
    fn write(&mut self, text: impl Into<String>) {
        self.segments.push(text.into());
    }
    fn write_opcode(&mut self, id: impl Into<String>) {
        let id = id.into();

        if !self.opcodes.contains(&id) {
            self.emit(OpcodeRegister { id: id.clone() });
        }

        self.write(
            (self
                .opcodes
                .iter()
                .enumerate()
                .find(|(_, text)| text == &&id)
                .unwrap()
                .0
                + 1)
            .to_string(),
        );
    }
    pub fn emit(&mut self, opcode: ContentScriptOpcode) {
        match opcode {
            OpcodeRegister { id } => {
                self.write_opcode("opcode_register");
                self.write(&id);
                self.opcodes.push(id);
            }
            BeatmapNote { time, lane } => {
                self.write_opcode("beatmap_note");
                self.write(time.to_string());
                self.write(lane.to_string());
            }
            BeatmapModifier { from, to, modifier } => {
                self.write_opcode("beatmap_modifier");
                self.write(from.to_string());
                self.write(to.to_string());
                self.write(modifier);
            }
            BeatmapCollectionRegister { name, script } => {
                self.write_opcode("beatmap_collection_register");
                self.write(name);
                self.write(script);
            }
            AddMenuItem {
                pos,
                next_state,
                state: text,
            } => {
                self.write_opcode("add_menu_item");
                self.write(pos.to_string());
                self.write(next_state);
                self.write(text);
            }
        }
    }

    pub fn text(&self) -> Result<String> {
        Ok(format!("V3{}", vec_to_string(self.segments.clone())?))
    }
}

pub fn encode(opcodes: Vec<ContentScriptOpcode>) -> Result<String> {
    let mut writer = ScriptWriter::new();

    for opcode in opcodes {
        writer.emit(opcode);
    }

    writer.text()
}
