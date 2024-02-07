use crc::Crc;
use std::convert::TryFrom;
use std::fmt;
use crate::Error;
use crate::chunk_type::ChunkType;
use std::array::TryFromSliceError;


const CRC: Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

pub struct Chunk {
    
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,

}

impl Chunk {

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Chunk {
            length: data.len() as u32,
            crc: CRC.checksum(&chunk_type.bytes().iter().cloned()
                                          .chain(data.iter().cloned())
                                          .collect::<Vec<u8>>()),
            chunk_type,
            data,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn data_as_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.data().to_vec())?)
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length().to_be_bytes().iter().cloned()
            .chain(self.chunk_type().bytes().iter().cloned())
            .chain(self.data().iter().cloned())
            .chain(self.crc().to_be_bytes().iter().cloned())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error; 

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {

        let data_len = bytes.len();
        let mut iter = bytes.iter().cloned();
        let first_four_bytes: [u8; 4] = iter.by_ref().take(4).collect::<Vec<u8>>().try_into().unwrap();
        let length = u32::from_be_bytes(first_four_bytes);

        let second_four_bytes: Vec<u8> = iter.by_ref().take(4).collect();
        let chunk_type = ChunkType::try_from(TryInto::<[u8; 4]>::try_into(second_four_bytes.as_slice()).unwrap()).unwrap();

        let data_bytes: Vec<u8> = iter.by_ref().take(data_len - 12).collect();

        let crc_byte: [u8; 4] = iter.collect::<Vec<u8>>().try_into().unwrap();
        let crc_from_slice = u32::from_be_bytes(crc_byte);

        let chunk = Chunk::new(chunk_type, data_bytes);


        if chunk.crc != crc_from_slice {
            return Err("Wrong crc".into());
        }

        Ok(chunk)

    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Length: {} Type: {} Crc: {}", self.length(), self.chunk_type(), self.crc())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}


