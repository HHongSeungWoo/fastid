use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

#[derive(Debug)]
pub struct ObjectId([u8; 12]);

impl ObjectId {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs() as u32;

        let mut rng = rand::thread_rng();
        let random_bytes = rng.gen::<[u8; 5]>();
        let counter = rng.gen::<[u8; 3]>();

        let mut bytes = [0; 12];
        bytes[0..4].copy_from_slice(&timestamp.to_be_bytes());
        bytes[4..9].copy_from_slice(&random_bytes);
        bytes[9..12].copy_from_slice(&counter);

        ObjectId(bytes)
    }

    pub fn to_hex_string(&self) -> String {
        hex::encode(self.0)
    }
}