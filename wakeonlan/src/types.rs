/// Represents a Wake-on-LAN magic packet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicPacket {
    /// The raw data of the magic packet
    pub data: Vec<u8>,
}

impl AsRef<[u8]> for MagicPacket {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
