pub mod crock_ford {

    use bytes::BytesMut;
    use lazy_static::lazy_static;
    use num_bigint::{BigUint, ToBigUint};
    use ring::rand::{SecureRandom, SystemRandom};

    const BYTE_SIZE: usize = 15;
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

    #[derive(Debug)]
    struct Bytes(BytesMut);

    impl Bytes {
        pub fn to_slice(&self) -> &[u8] {
            &self.0[..]
        }

        pub fn to_int(&self) -> BigUint {
            BigUint::from_bytes_be(&self.0[..])
        }

        pub fn to_vec(&self) -> Vec<u8> {
            self.0.to_vec()
        }

        pub fn derive_crockford_checksum(&self) -> BigUint {
            self.to_int() % ToBigUint::to_biguint(&CROCKFORD_MODULO_PRIME).unwrap()
        }

        pub fn new(size: usize) -> Result<Self, String> {
            let mut bytes = vec![0; size];
            rng().fill(&mut bytes).map_err(|e| e.to_string())?;
            Ok(Self(BytesMut::from_iter(bytes.iter())))
        }
    }

    impl TryFrom<BigUint> for Bytes {
        type Error = &'static str;
        fn try_from(value: BigUint) -> Result<Self, Self::Error> {
            let bytes = value.to_bytes_be();
            Bytes::try_from(bytes)
        }
    }

    impl TryFrom<Vec<u8>> for Bytes {
        type Error = &'static str;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            let bytes = BytesMut::try_from(&value[..])
                .map_err(|_| "unable to convert value to mutable byte")?;
            Ok(Self(bytes))
        }
    }

    #[derive(Debug)]
    pub struct Uuid {
        bytes: Bytes,
        checksum: BigUint,
    }

    impl Uuid {
        pub fn new() -> Self {
            let bytes = Bytes::new(BYTE_SIZE).expect("failed to generate random bytes");
            let checksum = bytes.derive_crockford_checksum();
            Self { bytes, checksum }
        }

        pub fn value(&self) -> String {
            base32::encode(base32::Alphabet::Crockford, &self.bytes.to_slice())
        }

        fn get_checksum_char(checksum: &BigUint) -> char {
            let checksum: i8 = checksum.try_into().unwrap();
            CROCKFORD_CHECKSUM_CHARS
                .chars()
                .nth(checksum.abs() as usize)
                .unwrap()
        }

        fn value_with_checksum(&self) -> String {
            format!(
                "{}{}",
                self.value(),
                Uuid::get_checksum_char(&self.checksum)
            )
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

            let id = &value[..=(Uuid::len() - 2)];
            let bytes = match base32::decode(base32::Alphabet::Crockford, id) {
                None => return Err("invalid uuid str"),
                Some(d) => Bytes::try_from(d)?,
            };

            let checksum = bytes.derive_crockford_checksum();
            if Uuid::get_checksum_char(&checksum)
                == value[(Uuid::len() - 1)..].chars().nth(0).unwrap()
            {
                Ok(Self { bytes, checksum })
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

    impl TryFrom<BigUint> for Uuid {
        type Error = &'static str;
        fn try_from(value: BigUint) -> Result<Self, Self::Error> {
            let bytes: Bytes = value
                .try_into()
                .map_err(|_| "unable to convert bigint to uuid")?;
            let checksum = bytes.derive_crockford_checksum();
            Ok(Self { bytes, checksum })
        }
    }

    impl Into<BigUint> for Uuid {
        fn into(self) -> BigUint {
            self.bytes.to_int()
        }
    }

    impl Into<Vec<u8>> for Uuid {
        fn into(self) -> Vec<u8> {
            self.bytes.to_vec()
        }
    }

    impl Into<Bytes> for Uuid {
        fn into(self) -> Bytes {
            self.bytes
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
    use num_bigint::BigUint;

    fn str_uuid() -> &'static str {
        "4s0y2vz7sf4vghnznytz9gvq6"
    }

    #[test]
    fn generate() {
        let uuid = Uuid::new();
        println!("uuid={}", uuid);
        assert_eq!(uuid.to_string().len(), 25); // 24 char identifier, 1 char checksum
    }

    #[test]
    fn generate_from_string() {
        let result: Uuid = str_uuid().try_into().unwrap();
        assert_eq!(result.to_string().to_lowercase(), str_uuid());
    }

    #[test]
    fn compare_two_uuid_of_same_value() {
        let first: Uuid = str_uuid().try_into().unwrap();
        let second: Uuid = str_uuid().try_into().unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn compare_two_uuid_of_diff_value() {
        let first: Uuid = str_uuid().try_into().unwrap();
        let second = Uuid::new();
        assert_ne!(first, second);
    }

    #[test]
    fn compare_uuid_with_string() {
        let uuid: Uuid = str_uuid().try_into().unwrap();
        assert_eq!(uuid, str_uuid().to_string());
    }

    #[test]
    fn get_uuid_as_integer_value() {
        let uuid: Uuid = str_uuid().try_into().unwrap();
        let int_value: BigUint = uuid.into();
        println!("{}", int_value);
    }

    // get as byte
    #[test]
    fn get_uuid_as_byte_value() {
        let uuid: Vec<u8> = Uuid::new().into();
        println!("{:?}", uuid);
    }

    // compare with int and byte
    #[test]
    fn convert_integer_to_uuid() {
        let int_value: BigUint = Uuid::try_from(str_uuid()).unwrap().try_into().unwrap();
        let uuid: Uuid = int_value.try_into().unwrap();
        assert_eq!(uuid, str_uuid().to_string())
    }
}
