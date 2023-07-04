// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::Value;
// use serde_json::Value::Number as VNumber;
use serde_json::Value::Object as VObject;
use serde_json::Value::String as VString;
use std::{
    error::Error,
    fs::{write, File},
    io::BufReader,
    path::{Path, PathBuf},
};
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Deserialize, Serialize, Debug, Default)]
struct Choice {
    name: String,
    picked: Value,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Spell {
    prep: String, // Is prepared
    name: String,
    save: String,
    time: String,
    range: String,
    comp: String, // Components
    duration: String,
    page_ref: String,
    notes: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Equipment {
    name: String,
    qty: i32,  // Quantity
    w: String, // Weight
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Atk {
    name: String,
    dmg: String,
    hit: String, //To hit bonus
    notes: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct ProfOption {
    mult: f32,    //multiplicator (1.0 for prof, 2.0 for expertise...)
    text: String, // H for half, P for prof and E for expertise
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Character {
    name: String,
    class_level: String,
    player_name: String,
    race: String,
    bg: String, // Background
    exp: i64,   // Experience
    str: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32,
    prof_mod: i32,     // Proficiency Bonus
    prof_st_str: bool, // Proficiency in strength saving throws
    prof_st_dex: bool,
    prof_st_con: bool,
    prof_st_int: bool,
    prof_st_wis: bool,
    prof_st_cha: bool,
    st_mods: String,  // Saving Throws modifiers
    acro: ProfOption, // Abilities (ex: Acrobatics)
    anim: ProfOption,
    arca: ProfOption,
    athl: ProfOption,
    dece: ProfOption,
    hist: ProfOption,
    insi: ProfOption,
    inti: ProfOption,
    inve: ProfOption,
    medi: ProfOption,
    natu: ProfOption,
    perc: ProfOption,
    perf: ProfOption,
    pers: ProfOption,
    reli: ProfOption,
    slei: ProfOption,
    stea: ProfOption,
    surv: ProfOption,
    ac: i32,     // Armor Class
    res: String, // Resistances and Immunities
    speeds: String,
    profs: String,
    max_hp: i32,
    hp: i32,
    temp_hp: i32,
    hd_total: String, // Total number of hit dice
    hd: String,       // Current number of hit dice
    ds_s_1: bool,     // Death saving throws - Success
    ds_s_2: bool,
    ds_s_3: bool,
    ds_f_1: bool, // Death saving throws - Failure
    ds_f_2: bool,
    ds_f_3: bool,
    actions: String,
    senses: String,
    atk_1: Atk,
    atk_2: Atk,
    atk_3: Atk,
    atk_4: Atk,
    atk_5: Atk,
    atk_6: Atk,
    features_1: String,
    features_2: String,
    features_3: String,
    cp: i32, // copper
    sp: i32, // silver
    ep: i32, // electrum
    gp: i32, // gold
    pp: i32, // platinum
    eq_1: Equipment,
    eq_2: Equipment,
    eq_3: Equipment,
    eq_4: Equipment,
    eq_5: Equipment,
    eq_6: Equipment,
    eq_7: Equipment,
    eq_8: Equipment,
    eq_9: Equipment,
    eq_10: Equipment,
    eq_11: Equipment,
    eq_12: Equipment,
    eq_13: Equipment,
    eq_14: Equipment,
    eq_15: Equipment,
    eq_16: Equipment,
    eq_17: Equipment,
    eq_18: Equipment,
    eq_19: Equipment,
    eq_20: Equipment,
    eq_21: Equipment,
    eq_22: Equipment,
    eq_23: Equipment,
    eq_24: Equipment,
    eq_25: Equipment,
    eq_26: Equipment,
    eq_27: Equipment,
    eq_28: Equipment,
    eq_29: Equipment,
    eq_30: Equipment,
    gender: String,
    age: String,
    size: String,
    height: String,
    weight: String,
    alig: String, // Alignment
    faith: String,
    skin: String,
    eyes: String,
    hair: String,
    appearance: String,
    allies: String,
    p_traits: String, // Personality traits
    ideals: String,
    bonds: String,
    flaws: String,
    backstory: String,
    notes_1: String,
    notes_2: String,
    sp_class: String, // Spellcasting class
    sp_ab: String,    // Spellcasting Ability
    sp_dc: String,    // Spell save DC
    sp_atk: String,   // spell attack bonus
    spells: Vec<Spell>,
}

//////////////////// functions

#[tauri::command]
fn get_empty_options() -> Vec<Value> {
    vec![]
}

#[tauri::command]
async fn open_options_file() -> Vec<Value> {
    let lineages = match FileDialogBuilder::new().pick_file() {
        Some(path_buffer) => read_options_from_file(path_buffer).unwrap_or_default(),
        None => Vec::<Value>::default(),
    };
    lineages
}

fn read_options_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Value>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let choices = serde_json::from_reader(reader)?;

    Ok(choices)
}

#[tauri::command]
fn get_default_character() -> Character {
    Character {
        name: "your character".to_string(),
        ..Default::default()
    }
}

#[tauri::command]
async fn open_character_file() -> Character {
    let c = match FileDialogBuilder::new().pick_file() {
        Some(fp) => read_character_from_file(fp).unwrap_or_default(),
        None => Character::default(),
    };
    c
}

fn read_character_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c = serde_json::from_reader(reader)?;

    Ok(c)
}

fn insert_between(list: Vec<String>, elem: String) -> String {
    let mut res = "".to_owned();
    for (i, s) in list.iter().enumerate() {
        res += s;
        if i != list.len() - 1 {
            res += &elem;
        }
    }
    res
}

#[tauri::command]
fn finish_building(mut c: Character, lineage_choices: Vec<Choice>) {
    c = fill_sheet(c, lineage_choices);
    save_character_to_file(c);
}

fn fill_sheet(mut c: Character, lineage_choices: Vec<Choice>) -> Character {
    let mut tool_profs = vec![];
    let mut languages = vec![];
    let mut speed_walk = 0;
    let mut speed_swim = 0;
    let mut speed_fly = 0;
    let mut speed_burrow = 0;
    let mut speed_climb = 0;

    for choice in lineage_choices {
        match choice.name.as_str() {
            "lineage" => {
                if let Some(VString(name)) = choice.picked.get("name") {
                    c.race = name.to_owned();
                }
                if let Some(VObject(scores)) = choice.picked.get("scores") {
                    if let Some(num) = scores["str"].as_i64() {
                        c.str += num as i32;
                    }
                    if let Some(num) = scores["dex"].as_i64() {
                        c.dex += num as i32;
                    }
                    if let Some(num) = scores["con"].as_i64() {
                        c.con += num as i32;
                    }
                    if let Some(num) = scores["int"].as_i64() {
                        c.int += num as i32;
                    }
                    if let Some(num) = scores["wis"].as_i64() {
                        c.wis += num as i32;
                    }
                    if let Some(num) = scores["cha"].as_i64() {
                        c.cha += num as i32;
                    }
                }
                if let Some(VObject(spd)) = choice.picked.get("speeds") {
                    if let Some(num) = spd["walk"].as_i64() {
                        speed_walk += num;
                    }
                    if let Some(num) = spd["climb"].as_i64() {
                        speed_climb += num;
                    }
                    if let Some(num) = spd["swim"].as_i64() {
                        speed_swim += num;
                    }
                    if let Some(num) = spd["fly"].as_i64() {
                        speed_fly += num;
                    }
                    if let Some(num) = spd["burrow"].as_i64() {
                        speed_burrow += num;
                    }
                }
            }

            "tool_prof_choice" => {
                if let VString(val) = choice.picked {
                    if val.as_str() != "none" {
                        tool_profs.push(val);
                    }
                }
            }

            // "draconic_ancestry" => {
            //     if let Some(picked) = choice.picked.get("name") {
            //         if picked.to_string().as_str() != "none" {
            //             c.profs += (picked.to_string() + "\n").as_str();
            //         }
            //     }
            // }
            "language_choice" => {
                if let VString(val) = choice.picked {
                    if val.as_str() != "none" {
                        languages.push(val);
                    }
                }
            }

            "skill_prof_choice_1" => {
                if let VString(val) = choice.picked {
                    if val.as_str() != "none" {
                        change_prof_by_code(&mut c, val[..4].to_lowercase(), 1.0)
                    }
                }
            }

            "skill_prof_choice_2" => {
                if let VString(val) = choice.picked {
                    if val.as_str() != "none" {
                        change_prof_by_code(&mut c, val[..4].to_lowercase(), 1.0)
                    }
                }
            }
            _ => {}
        }
    }

    // Handle proficiencies and languages
    c.profs = "".to_string();
    c.profs.push_str("=== Tools ===\n");
    c.profs
        .push_str(&insert_between(tool_profs, ", ".to_owned()));
    c.profs.push('\n');
    c.profs.push('\n');
    c.profs.push_str("=== Languages ===\n");
    c.profs
        .push_str(&insert_between(languages, ", ".to_owned()));
    c.profs.push('\n');
    c.profs.push('\n');

    c
}

fn change_prof_by_code(c: &mut Character, skill_code: String, mult: f32) {
    let skill = get_skill_by_code(c, skill_code.clone());
    match skill {
        Ok(s) => {
            set_prof(s, mult);
        }
        Err(msg) => {
            println!("{}", msg);
            println!("{}", skill_code);
        }
    }
}

fn set_prof(skill: &mut ProfOption, mult: f32) {
    skill.mult = mult;
    match mult {
        x if x == 0.5 => {
            skill.text = "H".to_string();
        }
        x if x == 1.0 => {
            skill.text = "P".to_string();
        }
        x if x == 2.0 => {
            skill.text = "E".to_string();
        }
        _ => {}
    }
}

fn get_skill_by_code(
    c: &mut Character,
    skill_code: String,
) -> Result<&mut ProfOption, &'static str> {
    match skill_code.as_str() {
        "acro" => Ok(&mut c.acro),
        "anim" => Ok(&mut c.anim),
        "arca" => Ok(&mut c.arca),
        "athl" => Ok(&mut c.athl),
        "dece" => Ok(&mut c.dece),
        "hist" => Ok(&mut c.hist),
        "insi" => Ok(&mut c.insi),
        "inti" => Ok(&mut c.inti),
        "inve" => Ok(&mut c.inve),
        "medi" => Ok(&mut c.medi),
        "natu" => Ok(&mut c.natu),
        "perc" => Ok(&mut c.perc),
        "perf" => Ok(&mut c.perf),
        "pers" => Ok(&mut c.pers),
        "reli" => Ok(&mut c.reli),
        "slei" => Ok(&mut c.slei),
        "stea" => Ok(&mut c.stea),
        "surv" => Ok(&mut c.surv),
        _ => Err("Wrong skill code!"),
    }
}

#[tauri::command]
fn save_character_to_file(c: Character) {
    let path = match FileDialogBuilder::new().save_file() {
        Some(path) => path,
        None => PathBuf::from(""), // this ensures failure at the next step if the user closed the file dialog
    };
    println!(
        "Saving status : {:?}",
        write(path, serde_json::to_string_pretty(&c).unwrap())
    );
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_empty_options,
            open_options_file,
            open_character_file,
            get_default_character,
            finish_building,
            save_character_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
