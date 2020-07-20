#![feature(proc_macro_hygiene)]
#![feature(asm)]

use rand::prelude::*;
use skyline::{hook, install_hooks, nn};
mod resource;
use resource::{PersonalData, WildPokemon};

const SPECIES_COUNT: u16 = 894;

#[hook(offset = 0x7709f0)]
pub unsafe fn wild_initialize(unk: u64, wild_pokemon: *mut WildPokemon) {
    let pokemon = &mut *wild_pokemon;
    let personal_data = PersonalData::get_instance();
    let mut rng = rand::thread_rng();

    loop {
        pokemon.species_id = rng.gen_range(0, SPECIES_COUNT as u32);
        let mut hp = personal_data.get(pokemon.species_id).unwrap().hp;

        if hp != 0 {
            break;
        }
    }

    let species_id = pokemon.species_id;

    let gender = personal_data.get(species_id).unwrap().gender;
    let form_count = personal_data.get(species_id).unwrap().form_count as u16;

    if gender == 0xFF {
        pokemon.gender = 2;
    } else {
        if gender == 0 {
            pokemon.gender = 0;
        } else if gender == 0xFE {
            pokemon.gender = 1;
        } else {
            pokemon.gender = rng.gen_range(0, 2);
        }
    }

    if form_count > 1 {
        pokemon.form_id = rng.gen_range(0, form_count);
    }

    pokemon.ability = rng.gen_range(0, 3);

    original!()(unk, pokemon);

    println!("{}", pokemon);
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

#[hook(offset = 0x18d18c0)]
pub fn kill_ldn_finalize() {
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
        kill_ldn_finalize,
        wild_initialize,
    );
}
