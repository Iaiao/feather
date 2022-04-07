use feather_api::components::{Sneaking, Sprinting};
use feather_api::protocol::packets::client::{EntityAction, EntityActionKind};

///  From [wiki](https://wiki.vg/Protocol#Entity_Action)
///  Sent by the client to indicate that it has performed certain actions:
///  *) sneaking (crouching),
///  *) sprinting,
///  *) exiting a bed,
///  *) jumping with a horse,
///  *) opening a horse's inventory while riding it.
///
pub fn handle_entity_action(
    packet: EntityAction,
    sneaking: &mut Sneaking,
    sprinting: &mut Sprinting,
) {
    match packet.action_id {
        EntityActionKind::StartSneaking | EntityActionKind::StopSneaking => {
            let start_sneaking = matches!(packet.action_id, EntityActionKind::StartSneaking);
            if **sneaking != start_sneaking {
                sneaking.0 = start_sneaking;
            }
        }
        EntityActionKind::LeaveBed => {
            //TODO issue #423
            // Note that the leave bed packet is not sent if the server changes night to day
            // and all players are kicked out of the bed. We have to separately send out
            // a notice that bed state might have changed.
        }
        EntityActionKind::StartSprinting | EntityActionKind::StopSprinting => {
            let start_sprinting = matches!(packet.action_id, EntityActionKind::StartSprinting);
            if **sprinting != start_sprinting {
                sprinting.0 = start_sprinting;
            }
        }
        EntityActionKind::StartHorseJump => {
            //TODO issue #423
        }
        EntityActionKind::StopJorseJump => {
            //TODO issue #423
        }
        EntityActionKind::OpenHorseInventory => {
            //TODO issue #423
        }
        EntityActionKind::StartElytraFlight => {
            //TODO issue #423
        }
    }
}
