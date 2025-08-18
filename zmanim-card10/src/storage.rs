use crate::gps_data::GpsData;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use esp_bootloader_esp_idf::partitions::{self, FlashRegion, PartitionEntry};
use esp_println::println;
use esp_storage::FlashStorage;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZmanimConfig {
    pub location: Option<GpsData>,
}

impl ZmanimConfig {
    pub fn new() -> Self {
        Self { location: None }
    }
    pub fn has_location(&self) -> bool {
        self.location.is_some() && self.location.as_ref().unwrap().has_position_fix()
    }
    pub fn with_location(&self, location: GpsData) -> Self {
        Self {
            location: Some(location),
        }
    }
}

pub struct ConfigStorage {
    offset_in_nvs_partition: u32,
}

impl ConfigStorage {
    pub fn new() -> Self {
        Self {
            offset_in_nvs_partition: 0,
        }
    }

    fn with_nvs_region<R>(&self, f: impl FnOnce(&mut FlashRegion<'_, FlashStorage>) -> R) -> R {
        let mut flash = FlashStorage::new();
        let mut pt_mem = [0u8; partitions::PARTITION_TABLE_MAX_LEN];
        let pt = partitions::read_partition_table(&mut flash, &mut pt_mem).unwrap();
        let nvs = pt
            .find_partition(partitions::PartitionType::Data(
                partitions::DataPartitionSubType::Nvs,
            ))
            .unwrap()
            .unwrap();

        let mut region = nvs.as_embedded_storage(&mut flash);
        f(&mut region)
    }
    fn with_nvs_partition<R>(&self, f: impl FnOnce(&mut PartitionEntry) -> R) -> R {
        let mut flash = FlashStorage::new();
        let mut pt_mem = [0u8; partitions::PARTITION_TABLE_MAX_LEN];
        let pt = partitions::read_partition_table(&mut flash, &mut pt_mem).unwrap();
        let mut nvs = pt
            .find_partition(partitions::PartitionType::Data(
                partitions::DataPartitionSubType::Nvs,
            ))
            .unwrap()
            .unwrap();
        f(&mut nvs)
    }

    pub fn wipe_config(&self) {
        self.with_nvs_partition(|nvs| {
            let mut flash = FlashStorage::new();
            flash.erase(nvs.offset(), nvs.offset() + nvs.len()).unwrap();
            println!("Wiped config");
        });
    }

    pub fn read_config(&self) -> Option<ZmanimConfig> {
        self.with_nvs_region(|region| {
            let mut bytes = [0u8; 100];
            region.read(self.offset_in_nvs_partition, &mut bytes).ok()?;
            let len_bytes = &bytes[..2];
            let len = u16::from_le_bytes([len_bytes[0], len_bytes[1]]) as usize;
            if len == 0 || len == 0xFFFF || len > bytes.len().saturating_sub(2) {
                return None;
            }
            let data = &bytes[2..2 + len];
            let deserialized_result = postcard::from_bytes(data);
            if deserialized_result.is_ok() {
                Some(deserialized_result.unwrap())
            } else {
                //Wipe the config
                region
                    .erase(
                        self.offset_in_nvs_partition,
                        self.offset_in_nvs_partition + 100,
                    )
                    .unwrap();
                None
            }
        })
    }

    pub fn write_config(&self, config: &ZmanimConfig) {
        self.with_nvs_region(|region| {
            let mut bytes = [0xFFu8; 100];
            let used = postcard::to_slice(config, &mut bytes[2..]).unwrap();
            let total_len = 2 + used.len();

            // Write length header (little-endian)
            let len_le = (used.len() as u16).to_le_bytes();
            bytes[0] = len_le[0];
            bytes[1] = len_le[1];

            let write_size = <FlashStorage as NorFlash>::WRITE_SIZE as usize;
            let erase_size = <FlashStorage as NorFlash>::ERASE_SIZE as u32;

            let padded_len = ((total_len + write_size - 1) / write_size) * write_size;

            let erase_to = ((padded_len as u32 + erase_size - 1) / erase_size) * erase_size;
            region
                .erase(
                    self.offset_in_nvs_partition,
                    self.offset_in_nvs_partition + erase_to,
                )
                .unwrap();

            region
                .write(self.offset_in_nvs_partition, &bytes[..padded_len])
                .unwrap();
        });
    }
}
