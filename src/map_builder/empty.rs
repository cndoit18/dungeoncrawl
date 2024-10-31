use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect;

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            monster_spawns: Vec::new(),
            theme: super::themes::DungeonTheme::new(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}
