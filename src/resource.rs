use skyline::hooks::{getRegionAddress, Region};
use std::fmt;
use std::sync::atomic::AtomicU32;

fn offset_to_addr(offset: usize) -> *const () {
    unsafe { (getRegionAddress(Region::Text) as *const u8).offset(offset as isize) as _ }
}

#[repr(C)]
pub struct CppVector<T> {
    start: *const T,
    end: *const T,
    eos: *const T,
}

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
    pub spe: u8,
    pub spa: u8,
    pub spd: u8,
    pub type_1: u8,
    pub type_2: u8,
    pub catch_rate: u8,
    pub evo_stage: u8,
    pub unk: [u8; 0xB0 - 0xA],
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
        self.table().get(species_id as usize).ok_or(PersonalDataError::SpeciesIdOutOfBounds)
    }

    pub fn get_mut(&mut self, species_id: u32) -> Result<&mut PersonalData, PersonalDataError> {
        self.table_mut().get_mut(species_id as usize).ok_or(PersonalDataError::SpeciesIdOutOfBounds)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PersonalDataError {
    SpeciesIdOutOfBounds
}

// Could serve as future reference on how to do things, do not remove yet


// #[repr(packed)]
// #[derive(Copy, Clone)]
// pub struct HashIndexGroup {
//     pub hash40: Hash40,
//     pub flags: [u8; 3],
// }

// impl fmt::Debug for HashIndexGroup {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "0x{:x}", self.hash40.as_u64())
//     }
// }

// #[repr(packed)]
// #[derive(Copy, Clone)]
// pub struct Hash40 {
//     crc32: u32,
//     len: u8,
// }

// impl fmt::Debug for Hash40 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "0x{:x}", self.as_u64())
//     }
// }

// impl Hash40 {
//     pub fn as_u64(&self) -> u64 {
//         (self.crc32 as u64) + ((self.len as u64) << 32)
//     }
// }
