use serde::{Deserialize, Serialize};
use serde_json::Result;
use aoe_djin::dat::DatFile;
use aoe_djin::dat::civilization::UnitType;
use aoe_djin::dat::tech::Tech;
use aoe_djin::dat::effect::EffectCommand;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let datfile =
        DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").expect("Error reading dat file");

    //datfile
    //    .civilization_table
    //    .civilizations
    //    .iter()
    //    .for_each(|civ| println!("{}", civ.name));
//
    /*datfile
        .tech_table
        .techs
        .iter()
        .for_each(
            |tech| println!("{}: {:?}",tech.name, tech.effect_id)
        );*/
    let mut buffer = File::create("foo.txt")?;

    /*fn get_effect(datfile : &DatFile, tech: &Tech, buffer :: &File) {
        match datfile
            .effect_table
            .effects
            .get(tech.effect_id as usize) {
            Some(x) => {
                buffer.write(format!("{:?}",tech.name));
                //println!("t: {}; e: {}", tech.name, x.name);
                //println!("{:?}", x.commands);
            },
            _ => {},
        }
    }*/
    fn commandsJson(efs: &Vec<EffectCommand>) -> String{
        let mut ret = String::from("[");
        efs
            .iter()
            .for_each(
                |efc| {
                    ret += &format!(
"
    <'command_type': {}, 'a': {:?}, 'b': {:?}, 'c': {:?}, 'd': {:?},>,",
                        efc.command_type, efc.a, efc.b, efc.c, efc.d);
                }
            );
        ret += &String::from("]");
        return ret
    }

    buffer.write(b"'TechEffects': [");
    datfile
        .tech_table
        .techs
        .iter()
        .for_each(
            |tech| {
                match datfile
                    .effect_table
                    .effects
                    .get(tech.effect_id as usize) {
                        Some(x) => {
                            buffer.write(format!(
"<
'techName': '{}',
'effectName': '{}',
'effects': {},>,
"
                                ,tech.name, x.name, commandsJson(&x.commands)
                            ).as_bytes());
                        },
                        _ => {},
                    }
            }
        );
    buffer.flush()?;
    Ok(())
    //let data = datfile.civilization_table.civilizations[0].
    //println!("{:?}", )
    /*
    datfile
        .effect_table
        .effects
        .iter()
        .for_each(
            |effect| println!("{}: {:?}",effect.name, effect.commands)
        );*/
    /*
    datfile
        .civilization_table
        .civilizations
        .iter()
        .take(1)
        .for_each(
            |civ| civ.units.iter().for_each(
                |unit| {
                    if unit.unit_type == UnitType::Creatable {
                        match (&unit.type_50, &unit.creatable) {
                            (Some(x), Some(y)) => {
                                println!("{}, misc: {:?}, {:?}", unit.name, x.reload_time, unit.hit_points);
                                println!("{}, armor : {:?}", unit.name, x.armor);
                                println!("{}, attack : {:?}", unit.name, x.attacks);
                                println!("{}, costs: {:?}", unit.name, y.resources_costs);
                            }
                            _ => {
                                println!("{:?}", unit.unit_type)
                            }
                        }
                    }
                    //if unit.unit_type == UnitType::Creatable {
                    //    println!("{:?}", unit.name)
                    //}    
                }
            )
        );*/
}
