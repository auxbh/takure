use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub call: CallStruct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStruct {
    pub playerdata_2: PlayerData2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData2 {
    pub data: GameScores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameScores {
    pub mode: String,
    #[serde(rename = "refid")]
    pub ref_id: String,
    #[serde(default)]
    pub isgameover: bool,
    #[serde(default)]
    pub note: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub stagenum: u8,
    pub mcode: u32,
    pub notetype: u8,
    pub clearkind: u8,
    pub score: u32,
    #[serde(rename = "exscore")]
    pub ex_score: u32,
    pub maxcombo: u32,
    pub fastcount: u32,
    pub slowcount: u32,
    pub judge_marvelous: u32,
    pub judge_perfect: u32,
    pub judge_great: u32,
    pub judge_good: u32,
    pub judge_miss: u32,
    pub judge_ok: u32,
    pub endtime: u128,
    pub playstyle: u8,
}