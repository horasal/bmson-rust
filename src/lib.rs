#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

impl Bmson {
    pub fn new(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl std::fmt::Display for Bmson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BMSON version: {}\n", self.version)?;
        write!(f, "Title : {}", self.info.title)?;
        if self.info.subtitle.len() > 0 {
            write!(f, " -{}-\n", self.info.subtitle)?;
        } else {
            write!(f, "\n")?; 
        }
        write!(f, "Artist: {}", self.info.artist)?;
        if let Some(ref subartists) = self.info.subartists {
            for subartist in subartists {
                write!(f, ", {}", subartist)?;
            }
        }
        write!(f, "\n")?;

        write!(f, "Genre: {}, Level: {}\n", self.info.genre, self.info.level)?;
        write!(f, "Total Notes: {}", self.sound_channels.iter().fold(0, |sum, x| sum + x.notes.len()))
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Bmson {
    version: String,
    info: BmsonInfo,
    lines: Option<Vec<BarLine>>,
    bpm_events: Option<Vec<BpmEvent>>,
    stop_event: Option<Vec<StopEvent>>,
    sound_channels: Vec<SoundChannel>,
    bga: Option<BGA>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BmsonInfo {
    title: String, subtitle: String,
    artist: String, subartists: Option<Vec<String>>,
    genre: String, mode_hint: String,
    chart_name: String,
    level: u64, init_bpm: f64,
    judge_rank: f64, total: f64,
    back_image: Option<String>,
    eyecatch_image: Option<String>,
    banner_image: Option<String>,
    preview_music: Option<String>,
    resolution: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BarLine {
    y: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SoundChannel {
    name: String,
    notes: Vec<Note>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    x: u64,
    y: u64,
    l: u64,
    c: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BpmEvent {
    y: u64,
    bpm: f64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct StopEvent {
    y: u64,
    duration: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BGA {
    bga_header: Vec<BGAHeader>,
    bga_events: Vec<BGAEvent>,
    layer_events: Vec<BGAEvent>,
    poor_events: Vec<BGAEvent>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BGAHeader{
    id: u64,
    name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BGAEvent {
    y: u64,
    id: u64,
}
