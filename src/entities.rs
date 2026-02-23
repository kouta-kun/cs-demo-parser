use source2_demo::Entity;
use crate::utils::OptionFrom;
use crate::utils::Variant;

#[derive(Clone)]
pub struct PawnEntity {
    pub handle: u32,
    pub entity_id: u32,
}

impl From<&Entity> for PawnEntity {
    fn from(pawn: &Entity) -> Self {
        let handle = pawn.handle();
        let entity_id = Variant::opt_from(
            pawn.get_property_by_name("m_nEntityId").unwrap()
        ).unwrap().u32();
        PawnEntity {
            handle,
            entity_id,
        }
    }
}

#[derive(Clone)]
pub struct ControllerEntity {
    pub handle: u32,
    pub pawn_handle: u32,
    pub player_name: String,
    pub steam_id: u64,
}

impl From<&Entity> for ControllerEntity {
    fn from(controller: &Entity) -> Self {
        let handle = controller.handle();
        let pawn_handle = Variant::opt_from(controller.get_property_by_name("m_hPawn").unwrap()).unwrap().u32();
        let player_name = Variant::opt_from(controller.get_property_by_name("m_iszPlayerName").unwrap()).unwrap().string();
        let steam_id = Variant::opt_from(controller.get_property_by_name("m_steamID").unwrap()).unwrap().u64();
        ControllerEntity {
            handle,
            pawn_handle,
            player_name,
            steam_id,
        }
    }
}