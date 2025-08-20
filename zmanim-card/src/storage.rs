use crate::gps_data::GpsData;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use esp_bootloader_esp_idf::partitions::{self, FlashRegion, PartitionEntry};
use esp_println::println;
use esp_storage::{FlashStorage, FlashStorageError};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZmanimConfig {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl ZmanimConfig {
    pub fn new() -> Self {
        Self {
            latitude: None,
            longitude: None,
        }
    }
    pub fn has_location(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }
    pub fn with_location(&self, latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: Some(latitude),
            longitude: Some(longitude),
        }
    }
}
pub struct Storage {
    fs: FlashStorage,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            fs: FlashStorage::new(),
        }
    }
    pub fn read(&mut self) -> Option<ZmanimConfig> {
        let mut read_buf = [0u8; 4096];
        self.fs
            .read(0x9000, &mut read_buf)
            .map(|_| postcard::from_bytes::<ZmanimConfig>(&read_buf).ok())
            .ok()?
    }
    pub fn write(&mut self, config: ZmanimConfig) {
        let serialized = postcard::to_allocvec(&config).unwrap();
        let mut buf: [u8; 4096] = [0; 4096];
        buf[..serialized.len()].copy_from_slice(&serialized);
        self.fs.write(0x9000, &buf).unwrap();
    }
}
