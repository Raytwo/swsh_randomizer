#![feature(proc_macro_hygiene)]
#![feature(asm)]

use rand::prelude::*;
use skyline::nn;
use skyline::{hook, install_hooks};
//mod bt;
mod resource;
use resource::PersonalData;
use std::fmt;

#[repr(C)]
pub struct WildPokemon {
    unk: [u8; 0x27],
    species_id: u32,
    form_id: u16,
    unk2: u16,
    gender: u16,
    nature: u16,
    ability: u8,
}

impl fmt::Display for WildPokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Pokemon #{}] Form: {}, Gender: {}, Nature {}, Ability: {}", self.species_id, self.form_id, self.gender, self.nature, self.ability)
    }
}

#[hook(offset = 0x7709f0)]
pub unsafe fn wild_initialize(unk: u64, wild_pokemon: *mut WildPokemon) {
    let pokemon = &mut *wild_pokemon;
    let personal_data = PersonalData::get_instance();
    let mut hp: u8;
    
    
    loop {
        pokemon.species_id = rand::thread_rng().gen_range(0, 893);
        hp = personal_data.get(pokemon.species_id).unwrap().hp;

        if hp != 0 { break; }
    }

    let spe = personal_data.get(pokemon.species_id).unwrap().speed;
    let spa = personal_data.get(pokemon.species_id).unwrap().spatk;
    let spd = personal_data.get(pokemon.species_id).unwrap().spdef;
    let gender = personal_data.get(pokemon.species_id).unwrap().gender;

    if gender == 0xFF {
        pokemon.gender = 2;
    } else {
        if gender == 0 {
            pokemon.gender = 0;
        } else if gender == 0xFE {
            pokemon.gender = 1;
        } else {
            pokemon.gender = rand::thread_rng().gen_range(0, 1);
        }
    }

    pokemon.ability = rand::thread_rng().gen_range(0, 2);
    
    original!()(unk, pokemon);

    println!("{}", pokemon);
    println!(
        "HP: {}, SPE: {}, SPA: {}, SPD: {}, Gender: {}",
        hp,
        spe,
        spa,
        spd,
        gender,
    );
}

#[hook(offset = 0x18d1750)]
pub fn kill_socket_initialize_config() {
    return;
}

#[hook(replace = nn::socket::Initialize)]
pub fn kill_socket_initialize() {
    return;
}

#[hook(replace = nn::socket::Finalize)]
pub fn kill_socket_finalize() {
    return;
}

/// Required for logging to work
#[hook(offset = 0x18d18d0)]
pub fn kill_ldn_initialize() {
    return;
}

#[skyline::main(name = "randomizer")]
pub fn main() {
    println!(
        "Randomizer v{} - Pokemon SwSh Wild Encounter Randomizer",
        env!("CARGO_PKG_VERSION")
    );
    install_hooks!(
        kill_socket_initialize,
        kill_socket_initialize_config,
        kill_socket_finalize,
        kill_ldn_initialize,
        wild_initialize,
    );
}
