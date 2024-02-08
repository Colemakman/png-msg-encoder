use std::fmt::{self, Display};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.to_owned()
    }

    pub fn is_critical(&self) -> bool {
        let first_byte: u8 = self.bytes[0];
        let fifth_bit = (first_byte >> 5) & 1;

        fifth_bit == 0 
    }

    pub fn is_public(&self) -> bool {
        let second_byte: u8 = self.bytes()[1];
        let fifth_bit = (second_byte >> 5) & 1;

        return fifth_bit == 0;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let third_byte: u8 = self.bytes()[2];
        let fifth_bit = (third_byte >> 5) & 1;

        return fifth_bit == 0;
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let fourth_byte: u8 = self.bytes()[3];
        let fifth_bit = (fourth_byte >> 5) & 1;

        return fifth_bit == 1;
    }
    
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            return Err("input str length is not 4".into());
        }
        let mut arr = [0; 4];
        arr.copy_from_slice(bytes);

        if s.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err("input str has non alphabetic ascii character(s)".into());
        }

        Ok(ChunkType { bytes: arr })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|byte| !byte.is_ascii_alphabetic()) {
            return Err("Value is not valid ascii alphabet");
        } 
        Ok(ChunkType { bytes: value })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               String::from_utf8(self.bytes().to_vec())
                   .expect("chunk_type should be valid utf-8")
                   .to_string()
                )
        }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        
        println!("Bytes: {:?}", chunk.bytes);
        println!("is_valid: {}", chunk.is_valid());

        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
