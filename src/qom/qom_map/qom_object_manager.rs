// Holds lists of the various objects
// until I figure a better way to sort it out

use crate::qom::qom_map::qom_object::qom_entrance_object::QomEntranceObject;
use crate::qom::qom_map::qom_object::qom_house_object::QomHouseObject;
use crate::qom::qom_map::qom_object::qom_npc_object::QomNpcObject;
use crate::qom::qom_map::qom_object::qom_player_object::QomPlayerObject;
use crate::qom::qom_map::qom_object::qom_sign_object::QomSignObject;
use crate::qom::qom_map::qom_object::qom_unknown_object::QomUnknownObject;

struct QomObjectManager {
    player: QomPlayerObject,
    unknown_objects: Vec<QomUnknownObject>,
    sign_objects: Vec<QomSignObject>,
    house_objects: Vec<QomHouseObject>,
    npc_objects: Vec<QomNpcObject>,
    entrance_objects: Vec<QomEntranceObject>,
}
