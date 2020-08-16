use std::{fmt, slice};
use std::io::{Cursor, Seek, SeekFrom};
use std::slice::Chunks;
use std::fs;
use std::io::prelude::*;

use byteorder::{ LittleEndian, BigEndian, ReadBytesExt };

use skyline::hooks::{getRegionAddress, Region};
use skyline::libc::{c_void};
use skyline::from_offset;
use skyline::logging::hex_dump_ptr;

fn offset_to_addr(offset: usize) -> *const () {
    unsafe { (getRegionAddress(Region::Text) as *const u8).offset(offset as isize) as _ }
}

#[repr(C)]
pub struct WildPokemon {
    unk: [u8; 0x27],
    pub species_id: u32,
    pub form_id: u16,
    unk2: [u16; 0x1],
    pub gender: u16,
    pub nature: u16,
    pub ability: u8,
}

impl fmt::Display for WildPokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Pokemon #{}] Form: {}, Gender: {}, Nature {}, Ability: {}",
            self.species_id, self.form_id, self.gender, self.nature, self.ability
        )
    }
}

#[from_offset(0x7711b0)]
pub fn calculate_checksum(dest: *const u8, size: usize) -> u64;

#[from_offset(0x771250)]
pub fn idk(dest: *const c_void, size: usize, encryption_constant: u32);

#[repr(C)]
pub struct Pk8Instance {
    pub vtable: *const (),
    pub unk: u64,
    pub pk8: *mut Pk8,
    unk2: [bool; 0x2],
}

impl Pk8Instance {
    pub unsafe fn get(&self) -> &Pk8 {
        &*self.pk8
    }

    pub unsafe fn get_mut(&self) -> &mut Pk8 {
        &mut *self.pk8
    }
}

#[repr(C)]
pub struct Pk8 {
    pub encryption_const: u32,
    pub sanity: u16,
    pub checksum: u16,
    pub species_id: u16,
    pub held_item: i16,
    pub tid: u16,
    pub sid: u16,
    pub exp: u32,
    pub ability: u16,
    unk: u16,
    pub mark_value: u16,
    unk2: u16,
    pub pid: u32,
    pub nature: u8,
    pub minted_nature: u8,
    pub gender: u8,
    unk3: u8,
    pub form_id: u16,
    pub evs: [u8;6],
    cnt_smth: [u8;6],
    pub pokerus: u8,
    unk4: [u8;29],
    pub height_scalar: u8,
    pub weight_scalar: u8,
    unk5: [u8;32],
    pub moves: [u16;4],
    pub moves_pp: [u8;4],
    pub moves_pp_plus: [u8;4],
    pub relearn_moves: [u16;4],
    pub current_hp: u16,
    pub iv32: u32,
    pub dynamax_level: u8,
    unk6: [u8;55],
    pub ht_friendship: u8,
    unk7: [u8;73],
    pub ot_friendship: u8,
    unk8: [u8;52],
}

impl Pk8 {
    pub unsafe fn refresh_checksum(&mut self)
    {
        println!("Valid checksum: {:#x}", self.checksum);

        let mut pk8_slice = as_bytes(self);
        let mut chksm: u64 = 0;

        for i in as_u16_iter(&pk8_slice[8..]) {
            chksm = (chksm as u32).wrapping_add(i as u32) as u64;
        }


        let test = &pk8_slice[pk8_slice.len() - 4..];
        println!("Print that out: {:#?}", test);

        //self.checksum = chksm;
        
        println!("New checksum: {:#x}", chksm);
    }
}

fn as_bytes<T: Sized>(x: &T) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts((x as *const T) as *const u8, core::mem::size_of::<T>())
    }
}

fn as_u16_iter<'a>(slice: &'a [u8]) -> impl Iterator<Item = u16> + 'a {
    slice.chunks_exact(2).map(|x| u16::from_le_bytes([x[0], x[1]]))
}

// protected override ushort CalculateChecksum()
// {
//     ushort chk = 0;
//     for (int i = 8; i < 328; i += 2)
//         chk += BitConverter.ToUInt16(Data, i);
//     return chk;
// }

#[repr(C)]
pub struct PersonalDataInstance {
    pub vtable: *const (),
    pub species_id: u32,
    pub form_id: u16,
    pub unk: u16,
    pub personal_data: *mut PersonalData,
}

#[repr(C)]
pub struct PersonalData {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub speed: u8,
    pub spatk: u8,
    pub spdef: u8,
    pub type_1: u8,
    pub type_2: u8,
    pub catch_rate: u8,
    pub evo_stage: u8,
    pub ev_yield: u16,
    pub item1: u16,
    pub item2: u16,
    pub item3: u16,
    pub gender: u8,
    pub hatch_cycles: u8,
    pub base_friendship: u8,
    pub exp_growth: u8,
    pub egg_group1: u8,
    pub egg_group2: u8,
    pub ability1: u16,
    pub ability2: u16,
    pub abilityhidden: u16,
    pub form_stat_index: u16,
    pub form_count: u8,
    pub color: u8,
    pub base_exp: u16,
    pub height: u16,
    pub weight: u16,
    pub unk1: [u8; 0x14],
    pub sprite_index: u16,
    pub unk2: [u8; 0xE],
    pub dex_id: u16,
    pub unk3: [u8; 0x62],
}

impl PersonalData {
    pub fn get_instance() -> &'static mut Self {
        unsafe {
            let instance_ptr: *mut &'static mut Self =
                std::mem::transmute(offset_to_addr(0x28b7d40));
            *instance_ptr
        }
    }

    pub fn table(&self) -> &[PersonalData] {
        unsafe { std::slice::from_raw_parts(PersonalData::get_instance(), 0x37D as usize) }
    }

    pub fn table_mut(&mut self) -> &mut [PersonalData] {
        unsafe { std::slice::from_raw_parts_mut(PersonalData::get_instance(), 0x37D as usize) }
    }

    pub fn get(&mut self, species_id: u32) -> Result<&PersonalData, PersonalDataError> {
        self.table()
            .get(species_id as usize)
            .ok_or(PersonalDataError::SpeciesIdOutOfBounds)
    }

    pub fn get_mut(&mut self, species_id: u32) -> Result<&mut PersonalData, PersonalDataError> {
        self.table_mut()
            .get_mut(species_id as usize)
            .ok_or(PersonalDataError::SpeciesIdOutOfBounds)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PersonalDataError {
    SpeciesIdOutOfBounds,
}
