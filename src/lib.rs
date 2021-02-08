#![feature(proc_macro_hygiene)]
#![feature(asm)]

use pokecrypto::encrypt_array8;
use rand::prelude::*;
use skyline::{hook, install_hooks, nn};
use skyline::logging::hex_dump_ptr;
mod resource;
use resource::{PersonalData, WildPokemon, Accessor};

use pkhexcore::pkm::{PKM, util::pokecrypto::SIZE_8PARTY};
use pkhexcore::{game::enums::species::Species, pkm::pk8::PK8};
use pkhexcore::pkm::util::pokecrypto;

use skyline::hooks::{getRegionAddress, Region};

const SPECIES_COUNT: u16 = 893;
use skyline::from_offset;

#[hook(offset = 0x7709f0)]
pub unsafe fn wild_initialize(pk8: *mut u8, wild_pokemon: *mut WildPokemon) {
    let pokemon = &mut *wild_pokemon;
    let personal_data = PersonalData::get_instance();
    let mut rng = rand::thread_rng();

    loop {
        pokemon.species_id = rng.gen_range(0..SPECIES_COUNT as u32);
        let mut hp = personal_data.get(pokemon.species_id).unwrap().hp;

        if hp != 0 {
            break;
        }
    }

    let species_id = pokemon.species_id;

    let gender = personal_data.get(species_id).unwrap().gender;
    let form_count = personal_data.get(species_id).unwrap().form_count as u16;

    pokemon.gender = match gender {
        0 => 0, // Male only
        0xFE => 1,// Female only
        0xFF => 2, // Genderless
        _ => rng.gen_range(0..2)
    };

    if form_count > 1 {
        pokemon.form_id = rng.gen_range(0..form_count);
    }

    pokemon.ability = rng.gen_range(0..3);
    original!()(pk8, pokemon);
}

#[hook(offset = 0x779360)]
pub unsafe fn set_species_id(accessor: &mut Accessor, species_id: u32) {
    original!()(accessor, species_id);
    let base = std::slice::from_raw_parts_mut(accessor.core_data, SIZE_8PARTY);
    let mut raw = base.to_vec();
    let mut pk8 = PK8::from(&raw);
    pk8.species = Species::Mew;
    let raw_pk8 = pk8.build();
    let mut pk8_bytes = raw_pk8.to_bytes();
    encrypt_array8(&mut pk8_bytes);
    base.copy_from_slice(pk8_bytes.as_slice());
}

#[hook(replace = nn::socket::Initialize_Config)]
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
#[hook(replace = nn::ldn::Initialize)]
pub fn kill_ldn_initialize() {
    return;
}

#[hook(replace = nn::ldn::Finalize)]
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
        //wild_initialize,
        set_species_id,
    );
}
