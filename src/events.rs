use std::collections::{HashMap, HashSet};
use crate::entities::{ControllerEntity, PawnEntity};
use crate::utils::Variant;

pub struct PlayerDeathEvent {
    pub killed: ControllerEntity,
    pub killer: Option<ControllerEntity>,
    pub weapon: String,
    pub attributes: HashSet<&'static str>,
    pub distance: f32,
}

impl PlayerDeathEvent {
    pub fn headshot(&self) -> bool {
        self.attributes.contains(&"headshot")
    }
    pub fn attacker_blind(&self) -> bool {
        self.attributes.contains(&"attackerblind")
    }
    pub fn victim_blind(&self) -> bool {
        self.attributes.contains(&"assistedflash")
    }
    pub fn noscope(&self) -> bool {
        self.attributes.contains(&"noscope")
    }
    pub fn thrusmoke(&self) -> bool {
        self.attributes.contains(&"thrusmoke")
    }

    pub fn description(&self) -> String {
        [
            if let Some(killer) = &self.killer {
                format!("{} killed {}", killer.player_name, self.killed.player_name)
            } else {
                format!("Self kill by {}", self.killed.player_name)
            },
            format!(", with {}", self.weapon),
            if self.headshot() {", headshot".to_string()} else {"".to_string()},
            if self.attacker_blind() {", attacker blinded".to_string()} else {"".to_string()},
            if self.victim_blind() {", victim blinded".to_string()} else {"".to_string()},
            if self.noscope() {", noscope".to_string()} else {"".to_string()},
            if self.thrusmoke() {", through a smoke".to_string()} else {"".to_string()},
            format!(", at a distance of {} units", self.distance)
        ].concat().to_string()
    }

    pub fn from(event_parsed: &HashMap<&str, Variant>, pawns: &HashMap<u32, PawnEntity>, controllers: &HashMap<u32, ControllerEntity>) -> Self {
        let killed = u32::from_ne_bytes(event_parsed.get(&"userid_pawn").unwrap().i32().to_ne_bytes());
        let killed_pawn = pawns.get(&killed).unwrap();
        let killed_controller = controllers.get(&killed_pawn.handle).unwrap().clone();

        let killer = u32::from_ne_bytes(event_parsed.get(&"attacker_pawn").unwrap().i32().to_ne_bytes());

        let killer_controller = if killer == u32::MAX {
            None
        } else {
            let killer_pawn = pawns.get(&killer).unwrap();

            let killer_controller = controllers.get(&killer_pawn.handle).unwrap();

            Some(killer_controller.clone())
        };

        let weapon = event_parsed.get(&"weapon").unwrap().string();
        let mut attributes = HashSet::new();
        for k in ["headshot", "attackerblind", "assistedflash", "noscope","thrusmoke"] {
            let attribute = event_parsed.get(&k).unwrap().bool();
            if attribute {
                attributes.insert(k);
            }
        }
        let distance = event_parsed.get(&"distance").unwrap().f32();

        PlayerDeathEvent {
            killed: killed_controller,
            killer: killer_controller,
            weapon,
            distance,
            attributes,
        }
    }
}