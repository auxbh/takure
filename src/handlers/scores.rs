use anyhow::{Ok, Result};
use crate::{helpers, CONFIGURATION, TACHI_IMPORT_URL};
use crate::types::game::GameScores;
use crate::types::tachi::{Difficulty, HitMeta, Import, ImportMeta, ImportScore, Judgements, Playtype, TachiLamp};
use log::{debug, info};

pub fn process_scores(scores: GameScores) -> Result<()> {
    if scores.isgameover {
        debug!("Aborting: isgameover is true");
        return Ok(());
    }

    if scores.ref_id.starts_with("X000") {
        info!("Guest play, skipping score(s) submission");
        return Ok(());
    }

    let card = if let Some(card) = helpers::get_current_card_id() {
        if !CONFIGURATION.cards.whitelist.is_empty()
            && !CONFIGURATION.cards.whitelist.contains(&card)
        {
            info!(
                "Card {} is not whitelisted, skipping score(s) submission",
                card
            );
            return Ok(());
        }

        card
    } else {
        info!("Card ID is not set, skipping score(s) submission");
        return Ok(());
    };

    let note = if let Some(highest_stage) = scores.note.iter()
        .filter(|n| n.stagenum != 0)
        .max_by_key(|n| n.stagenum)
        .cloned()
    {
        debug!("Selected note with highest stagenum != 0: {:#?}", highest_stage);
        highest_stage
    } else {
        debug!("No valid note with stagenum != 0 found, skipping");
        return Ok(());
    };

    if note.playstyle == 2 {
        info!("Versus play, skipping score(s) submission");
        return Ok(());
    }

    let import_score = ImportScore {
        score: note.score,
        lamp: TachiLamp::from(note.clearkind),
        match_type: "inGameID".to_string(),
        identifier: note.mcode.to_string(),
        difficulty: Difficulty::from(note.notetype),
        time_achieved: note.endtime,
        judgements: Judgements {
            marvelous: note.judge_marvelous,
            perfect: note.judge_perfect,
            great: note.judge_great,
            good: note.judge_good,
            miss: note.judge_miss,
            ok: note.judge_ok,
        },
        hit_meta: HitMeta {
            fast: note.fastcount,
            slow: note.slowcount,
            max_combo: note.maxcombo,
            ex_score: note.ex_score,
        },
    };

    let import_meta = ImportMeta {
        game: "ddr".to_string(),
        play_type: Playtype::from(note.playstyle),
        service: "Takure".to_string(),
    };

    let import = Import {
        meta: import_meta,
        scores: vec![import_score],
    };

    if cfg!(debug_assertions) {
        debug!("Tachi API request data: {:#?}", import);
    } else {
        helpers::call_tachi("POST", TACHI_IMPORT_URL.as_str(), Some(import))?;
    }
    info!("Successfully imported score(s) for card {}", card);

    Ok(())
}
