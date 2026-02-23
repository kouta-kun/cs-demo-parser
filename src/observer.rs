use source2_demo::Observer;
use std::collections::HashMap;
use std::iter::zip;
use source2_demo::{observer, on_message, uses_entities, uses_game_events, uses_string_tables, Context, ObserverResult};
use source2_demo::proto::c_msg_source1_legacy_game_event_list::DescriptorT;
use source2_demo::prelude::*;
use source2_demo::proto::*;
use crate::entities::{ControllerEntity, PawnEntity};
use crate::events::PlayerDeathEvent;
use crate::utils::{OptionFrom, Variant};

// Create a struct that implements the Default trait
#[derive(Default)]
pub struct DemoObserver {
    descriptors: HashMap<i32, DescriptorT>,
    round: u64,
    round_start: u32,
    collected_pawns: bool,
    pawn_ids: HashMap<u32, PawnEntity>,
    controller_ids: HashMap<u32, ControllerEntity>,

    find_chat: bool,
    filter: String,

    find_kills: bool,
    killer_name: Option<String>,
    killed_name: Option<String>,
    weapon: Option<String>,
    filter_attributes: Vec<String>,

    programmatic_output: bool,
}

// Mark the impl block with the observer attribute
#[observer(all)]
#[uses_game_events]
#[uses_entities]
#[uses_string_tables]
impl DemoObserver {

    #[on_message]
    fn handle_demo_msg_list(
        &mut self,
        _ctx: &Context,
        msg: CMsgSource1LegacyGameEventList,
    ) -> ObserverResult {
        for desc in msg.descriptors {
            self.descriptors.insert(desc.eventid(), desc);
        }
        Ok(())
    }

    #[on_message]
    fn handle_demo_msg(
        &mut self,
        ctx: &Context,
        msg: CMsgSource1LegacyGameEvent,
    ) -> ObserverResult {
        let descriptor = self.descriptors.get(&msg.eventid()).unwrap();
        let event_name = descriptor.name();

        if event_name.contains("round_officially_ended") {
            self.round_start = ctx.tick();
            self.round += 1
        } else if event_name == ("player_death") {
            if !self.find_kills {
                return Ok(())
            }
            if !self.collected_pawns {
                self.collected_pawns = true;
                let player_controllers: Vec<_> = ctx.entities().iter().filter(|e| e.class().name() == "CCSPlayerController").collect();
                let player_pawns: Vec<_> = ctx.entities().iter().filter(|e| e.class().name() == "CCSPlayerPawn").collect();
                self.pawn_ids = player_pawns.into_iter().map(|pawn| {
                    let pawn = PawnEntity::from(pawn);
                    (pawn.entity_id, pawn)
                }).collect();
                self.controller_ids = player_controllers.into_iter().map(|controller| {
                    let controller = ControllerEntity::from(controller);
                    (controller.pawn_handle, controller)
                }).collect();
            }

            let event_keys: HashMap<_, _> = zip(&descriptor.keys, msg.keys).map(|(key,value)| {
                let name = key.name();
                let value = Variant::opt_from(&value).unwrap();
                (name, value)
            }).collect();

            let event = PlayerDeathEvent::from(&event_keys, &self.pawn_ids, &self.controller_ids);
            let end_tick = ctx.tick() + 64;
            let start_tick = ctx.tick().saturating_sub(64 * 5).max(self.round_start);
            let seconds = (ctx.tick() as f32) / 64f32;
            let minutes = seconds as u64 / 60;
            let seconds = (seconds as u64) % 60;
            if let Some(weapon) = &self.weapon {
                if !event.weapon.contains(weapon) {
                    return Ok(())
                }
            }
            if self.filter_attributes.len() > 0 {
                if self.filter_attributes.iter().any(|a| !event.attributes.contains(a.as_str())) {
                    return Ok(())
                }
            }
            if let Some(killed) = &self.killed_name {
                if event.killed.player_name != *killed {
                    return Ok(())
                }
            }
            if let Some(killer) = &self.killer_name {
                if event.killer.is_none() || event.killer.as_ref().unwrap().player_name != *killer {
                    return Ok(())
                }
            }
            if self.programmatic_output {
                println!("{start_tick},{end_tick},{}",
                         event.killer.unwrap_or(event.killed).player_name,
                );
            } else {
                println!(
                    "[Round {}, {minutes}:{seconds}]: {}",
                    self.round+1,
                    event.description(),
                )
            }
        }
        Ok(())
    }

    #[on_message]
    fn handle_header(
        &mut self,
        _ctx: &Context,
        header: CDemoFileHeader,
    ) -> ObserverResult {
        println!("Map: {}", header.map_name());
        Ok(())
    }

    // Use the on_message attribute to mark the protobuf message handler
    #[on_message]
    fn handle_chat_msg_2(
        &mut self,
        ctx: &Context,
        chat_msg: CUserMessageSayText2, // Use any protobuf message as an argument
    ) -> ObserverResult {
        if !self.find_chat {
            return Ok(())
        }
        let end_tick = ctx.tick() + 64;
        let start_tick = ctx.tick().saturating_sub(64 * 15).max(self.round_start);
        let seconds = (ctx.tick() as f32) / 64f32;
        let minutes = seconds as u64 / 60;
        let seconds = (seconds as u64) % 60;
        let chat_content = chat_msg.param2();
        if chat_content.contains(&self.filter) {
            if self.programmatic_output {
                println!("{start_tick},{end_tick},{}",
                         chat_msg.param1(),
                );
            } else {
                println!(
                    "[Round {}, {minutes}:{seconds}] {}: {}",
                    self.round+1,
                    chat_msg.param1(),
                    chat_content,
                )
            }
        }
        Ok(())
    }

    pub fn filter(&mut self, filter: String) -> &mut Self {
        self.filter = filter;
        self
    }

    pub fn programmatic_output(&mut self, verbose: bool) -> &mut Self {
        self.programmatic_output = verbose;
        self
    }

    pub fn filter_attributes(&mut self, filter_attributes: Vec<String>) -> &mut Self {
        self.filter_attributes = filter_attributes;
        self
    }

    pub fn weapon(&mut self, weapon: Option<String>) -> &mut Self {
        self.weapon = weapon;
        self
    }

    pub fn killed_name(&mut self, killed_name: Option<String>) -> &mut Self {
        self.killed_name = killed_name;
        self
    }

    pub fn killer_name(&mut self, killer_name: Option<String>) -> &mut Self {
        self.killer_name = killer_name;
        self
    }

    pub fn find_kills(&mut self, find_kills: bool) -> &mut Self {
        self.find_kills = find_kills;
        self
    }

    pub fn find_chat(&mut self, find_chat: bool) -> &mut Self {
        self.find_chat = find_chat;
        self
    }
}