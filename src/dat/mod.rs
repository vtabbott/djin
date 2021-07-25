mod zlib;

use crate::dat::civilization::Civilizations;
use crate::dat::color::ColorTable;
use crate::dat::effect::Effects;
use crate::dat::random_map::RandomMap;
use crate::dat::sound::SoundTable;
use crate::dat::sprite::SpriteTable;
use crate::dat::tech::Techs;
use crate::dat::tech_tree::TechTree;
use crate::dat::terrain::{TerrainHeader, TerrainRestrictions};
use crate::dat::terrain_block::TerrainBlock;
use crate::dat::unit::Units;
use djin_protocol::Parcel;
use eyre::Result;
use std::path::Path;

mod civilization;
mod color;
mod common;
mod effect;
mod random_map;
mod sound;
mod sprite;
mod tech;
mod tech_tree;
mod terrain;
mod terrain_block;
mod unit;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct ResourceUsage {
    /// The kind of resource to give or take
    pub attribute: ResourceUsageType,
    /// The amount give or take
    pub amount: i16,
    /// How and when this is counted
    pub flag: ResourceUsageTrigger,
}

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u16)]
pub enum ResourceUsageTrigger {
    OnCreate = 0,
    OnQueue = 1,
}

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u16)]
pub enum ResourceUsageType {
    /// Take or give an amount of food to the player
    Food = 0,
    /// Take or give an amount of wood to the player
    Wood = 1,
    /// Take or give an amount of stone to the player
    Stone = 2,
    /// Take or give an amount of gold to the player
    Gold = 3,
    /// Take or give an amount of population to the player
    Pop = 4,
    /// A free unit (Elite Kipchak)
    Free = 214,
    /// Two units in the game use this attribute : Elite Kipchak and Urus Khan (migth be creatable on some campaingn scenario)
    DecreaseSharedUnitCount = 215,
    /// A town center slot either in dark age (UNKOWN RTWC1X) or in feudal age for Cumans (UNKOWN RTWC2X)
    TownCenter = 218,
    /// Also for Elite Kipchak and Urus Khan, decrease the number of available unit (10 For Kipchak)
    TeamBonusCounter,
    // We cannot use i16 as enum discriminant but this is actually -1
    /// This can be ignored
    None = 65535,
}

pub struct DatFile {
    pub game_version: GameVersion,
    pub terrain_header: TerrainHeader,
    pub terrain_restrictions: TerrainRestrictions,
    pub color_table: ColorTable,
    pub sound_table: SoundTable,
    pub sprite_table: SpriteTable,
    pub terrain_block: TerrainBlock,
    pub random_map: RandomMap,
    pub effects: Effects,
    pub units: Units,
    pub civilizations: Civilizations,
    pub techs: Techs,
    pub misc: Misc,
    pub tech_tree: TechTree,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct GameVersion {
    #[protocol(fixed_length(8))]
    pub game_version: String,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Misc {
    time_slice: u32,
    unit_kill_rate: u32,
    unit_kill_total: u32,
    unit_hit_point_rate: u32,
    unit_hit_point_total: u32,
    razing_kill_rate: u32,
    razing_kill_total: u32,
}

impl DatFile {
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<DatFile> {
        let mut buf = zlib::decompress(path)?;

        let settings = djin_protocol::Settings {
            byte_order: djin_protocol::ByteOrder::LittleEndian,
        };

        let game_version = GameVersion::read(&mut buf, &settings).expect("Read error");
        let terrain_header = TerrainHeader::read(&mut buf, &settings).expect("Read error");
        let terrain_restrictions = TerrainRestrictions::read(
            &mut buf,
            terrain_header.terrain_restriction_size as usize,
            terrain_header.restriction_size as usize,
            &settings,
        );
        let color_table = ColorTable::read(&mut buf, &settings).expect("Read error");
        let sound_table = SoundTable::read(&mut buf, &settings).expect("Read error");
        let sprite_table = SpriteTable::read(&mut buf, &settings).expect("Read error");
        let terrain_block = TerrainBlock::read(&mut buf, &settings).expect("Read error");
        let random_map = RandomMap::read(&mut buf, &settings).expect("Read error");
        let effects = Effects::read(&mut buf, &settings).expect("Read error");
        let units = Units::read(&mut buf, &settings).expect("Read error");
        let civilizations = Civilizations::read(&mut buf, &settings).expect("Read error");
        let techs = Techs::read(&mut buf, &settings).expect("Read error");
        let misc = Misc::read(&mut buf, &settings).expect("Read error");
        let tech_tree = TechTree::read(&mut buf, &settings).expect("Read error");

        Ok(DatFile {
            game_version,
            terrain_header,
            terrain_restrictions,
            color_table,
            sound_table,
            sprite_table,
            terrain_block,
            random_map,
            effects,
            units,
            civilizations,
            techs,
            misc,
            tech_tree,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::dat::tech::ResourceCostType;
    use crate::dat::tech::{ResourceCostTrigger, TechResourcesCost};
    use crate::dat::DatFile;
    use eyre::Result;
    use spectral::prelude::*;

    type TestResult = Result<()>;

    #[test]
    fn should_read_dat_file() -> TestResult {
        let dat_file = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").unwrap();
        // Version
        assert_that(&dat_file.game_version.game_version).is_equal_to("VER 7.4\0".to_string());

        // Terrain Header
        assert_that(&dat_file.terrain_header.terrain_restriction_size).is_equal_to(31);
        assert_that(&dat_file.terrain_header.restriction_size).is_equal_to(110);
        assert_that(&dat_file.terrain_header.terrain_restriction_size).is_equal_to(31);
        assert_that(&dat_file.terrain_header.terrain_tables_pointer).has_length(31);
        assert_that(&dat_file.terrain_header.terrains_pointer).has_length(31);

        // Terrain restrictions
        assert_that(&dat_file.terrain_restrictions.inner).has_length(31);

        dat_file.terrain_restrictions.inner.iter().for_each(|el| {
            assert_that(&el.pass_graphics).has_length(110);
            assert_that(&el.passability).has_length(110);
        });

        // Colors
        assert_that(&dat_file.color_table.colors).has_length(16);
        assert_that(&dat_file.sound_table.sounds).has_length(685);
        assert_that(&dat_file.civilizations.civilizations).has_length(38);

        let fletching = dat_file
            .techs
            .techs
            .iter()
            .find(|tech| tech.name == "Fletching")
            .expect("Could not find fletching");

        // Fletching cost 100 Food and 50 gold
        assert_that(&fletching.research_resource_cost).contains_all_of(
            &vec![
                TechResourcesCost {
                    amount: 100,
                    flag: ResourceCostTrigger::OnQueue,
                    resource_type: ResourceCostType::Food,
                },
                TechResourcesCost {
                    amount: 50,
                    flag: ResourceCostTrigger::OnQueue,
                    resource_type: ResourceCostType::Gold,
                },
                TechResourcesCost {
                    amount: 0,
                    flag: ResourceCostTrigger::OnCreate,
                    resource_type: ResourceCostType::None,
                },
            ]
            .iter(),
        );

        Ok(())
    }
}
