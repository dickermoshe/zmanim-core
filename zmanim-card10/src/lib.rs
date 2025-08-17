#![no_std]

pub mod gps;
pub mod gps_data;

use heapless::String;

pub fn string_from_buffer<const N: usize>(buffer: &[u8]) -> String<N> {
    let mut str = String::<N>::new();
    for i in 0..buffer.len() {
        str.push(buffer[i] as char).unwrap();
    }
    str
}
