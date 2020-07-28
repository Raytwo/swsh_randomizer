#![feature(proc_macro_hygiene)]
#![feature(asm)]

use rand::prelude::*;
use skyline::{hook, install_hooks, nn};
use skyline::logging::hex_dump_ptr;
mod resource;
use resource::{PersonalData, WildPokemon, Pk8, calculate_checksum, idk};

const SPECIES_COUNT: u16 = 893;

#[hook(offset = 0x7709f0)]
pub unsafe fn wild_initialize(pk8: *mut Pk8, wild_pokemon: *mut WildPokemon) {
    let pokemon = &mut *wild_pokemon;
    let personal_data = PersonalData::get_instance();
    let mut rng = rand::thread_rng();
    let pk8_temp = &mut *pk8;

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

    pokemon.gender = match gender {
        0 => 0, // Male only
        0xFE => 1,// Female only
        0xFF => 2, // Genderless
        _ => rng.gen_range(0, 2)
    };

    if form_count > 1 {
        pokemon.form_id = rng.gen_range(0, form_count);
    }

    pokemon.ability = rng.gen_range(0, 3);
    original!()(pk8, pokemon);

    
    //pk8_temp.species_id = 150;
    pk8_temp.refresh_checksum();
    // calculate_hash((pk8 as *const u8).offset(8) as *const skyline::libc::c_void, 0x140);
    // let chksm = calculate_checksum((pk8 as *const u8).offset(8) as *const skyline::libc::c_void, 0x140);

    // if pk8_temp.checksum != chksm {
    //     pk8_temp.sanity = pk8_temp.sanity | 4;
    // }

    //pk8_temp.checksum = calculate_checksum((pk8 as *const u8).offset(8) as *const skyline::libc::c_void, 0x140);

    //idk((pk8 as *const u8).offset(8) as *const skyline::libc::c_void, 0x140, pk8_temp.encryption_const);
    println!("{}, Pk8 PID: {}", pokemon, pk8_temp.pid);
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
        wild_initialize,
    );
}
