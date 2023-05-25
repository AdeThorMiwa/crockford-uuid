pub mod crock_ford {
    use lazy_static::lazy_static;
    use ring::rand::{SecureRandom, SystemRandom};

    const BYTE_SIZE: usize = 20;
    const CROCKFORD_CHECKSUM_CHARS: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ*~$=U";
    // a prime number greater than 32 for checksum derivation
    const CROCKFORD_MODULO_PRIME: usize = 37;

    fn rng() -> &'static dyn SecureRandom {
        use std::ops::Deref;

        lazy_static! {
            static ref RANDOM: SystemRandom = SystemRandom::new();
        }

        RANDOM.deref()
    }

    fn rand_bytes(size: usize) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = vec![0; size];
        rng().fill(&mut bytes).map_err(|e| e.to_string())?;
        Ok(bytes)
    }

    #[derive(Debug)]
    pub struct Uuid {
        bytes: Vec<u8>,
        checksum: i8,
    }

    impl Uuid {
        pub fn new() -> Self {
            let bytes = rand_bytes(BYTE_SIZE).expect("failed to generate random bytes");
            let checksum = Uuid::derive_checksum(&mut &bytes[..]);
            Self { bytes, checksum }
        }

        pub fn value(&self) -> String {
            base32::encode(base32::Alphabet::Crockford, &self.bytes)
        }

        fn derive_checksum(bytes: &mut &[u8]) -> i8 {
            let (int_bytes, rest) = bytes.split_at(std::mem::size_of::<i128>());
            *bytes = rest;
            let id_to_int = i128::from_ne_bytes(int_bytes.try_into().unwrap());
            (id_to_int % CROCKFORD_MODULO_PRIME as i128) as i8
        }

        fn get_checksum_char(checksum: i8) -> char {
            CROCKFORD_CHECKSUM_CHARS
                .chars()
                .nth(checksum.abs() as usize)
                .unwrap()
        }

        fn value_with_checksum(&self) -> String {
            format!("{}{}", self.value(), Uuid::get_checksum_char(self.checksum))
        }

        fn len() -> usize {
            // we are trying to fit 8 bits bytes into a 5 bit char
            (BYTE_SIZE * 8 / 5) + 1
        }

        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() != Uuid::len() {
                return Err("invalid string length");
            }

            let value = value.to_ascii_uppercase();

            let id = &value[..=31];
            let decoded = match base32::decode(base32::Alphabet::Crockford, id) {
                None => return Err("invalid uuid str"),
                Some(d) => d,
            };

            let derived_cksum = Uuid::derive_checksum(&mut &decoded[..]);
            if Uuid::get_checksum_char(derived_cksum) == value[32..].chars().nth(0).unwrap() {
                Ok(Self {
                    bytes: decoded,
                    checksum: derived_cksum,
                })
            } else {
                Err("invalid uuid str")
            }
        }
    }

    impl std::fmt::Display for Uuid {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value_with_checksum())
        }
    }

    impl TryFrom<&str> for Uuid {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Uuid::from_str(value)
        }
    }

    impl TryFrom<String> for Uuid {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            Uuid::from_str(value.as_str())
        }
    }

    impl PartialEq<Uuid> for Uuid {
        fn eq(&self, other: &Uuid) -> bool {
            self.value_with_checksum() == other.value_with_checksum()
        }
    }

    impl PartialEq<String> for Uuid {
        fn eq(&self, other: &String) -> bool {
            match Uuid::from_str(other) {
                Ok(uuid) => *self == uuid,
                Err(_) => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crock_ford::Uuid;

    #[test]
    fn generate() {
        let uuid = Uuid::new();
        assert_eq!(uuid.to_string().len(), 33); // 32 char identifier, 1 char checksum
    }

    #[test]
    fn generate_from_string() {
        let str_uuid = "1fe1ewyb60gvfj71yd4aq1qftz5dkwkjg";
        let result: Uuid = str_uuid.try_into().unwrap();
        assert_eq!(result.to_string().to_lowercase(), str_uuid);
    }

    #[test]
    fn compare_two_uuid_of_same_value() {
        let str_uuid = "1fe1ewyb60gvfj71yd4aq1qftz5dkwkjg";
        let first: Uuid = str_uuid.try_into().unwrap();
        let second: Uuid = str_uuid.try_into().unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn compare_two_uuid_of_diff_value() {
        let str_uuid = "1fe1ewyb60gvfj71yd4aq1qftz5dkwkjg";
        let first: Uuid = str_uuid.try_into().unwrap();
        let second = Uuid::new();
        assert_ne!(first, second);
    }

    #[test]
    fn compare_uuid_with_string() {
        let str_uuid = "1fe1ewyb60gvfj71yd4aq1qftz5dkwkjg";
        let uuid: Uuid = str_uuid.try_into().unwrap();
        assert_eq!(uuid, str_uuid.to_string());
    }

    // get as integer

    // get as byte

    // compare with int and byte
}
