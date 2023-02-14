use ethereum_types::{H160, U256};

pub fn convert_u256_to_eth_address(value: U256) -> H160 {
    let mut be_u256 = [0u8; 32];
    value.to_big_endian(&mut be_u256);
    H160::from_slice(&be_u256[12..])
}

pub fn compressed_u256_bytes(value: U256) -> Vec<u8> {
    let mut compressed = vec![];

    let mut value_be = vec![0u8; 32];
    value.to_big_endian(&mut value_be);

    for i in value_be {
        if i != 0 {
            compressed.push(i);
        }
    }

    return compressed;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_convert_u256_as_address() {
        let value = U256::from_str_radix(
            "0x00000000000000000000000073F8A075b9a1e3ddD169CfdBdFA513c40B8bd796",
            16,
        )
        .unwrap();

        assert_eq!(
            convert_u256_to_eth_address(value),
            H160::from_str("0x73F8A075b9a1e3ddD169CfdBdFA513c40B8bd796").unwrap()
        )
    }

    #[test]
    fn test_compressed_u256_bytes() {
        let compressed = compressed_u256_bytes(U256::from(0x2077));
        assert_eq!(compressed.len(), 2);
        assert_eq!(compressed[0], 0x20);
        assert_eq!(compressed[1], 0x77);

        let compressed = compressed_u256_bytes(U256::from(0x207769));
        assert_eq!(compressed.len(), 3);
        assert_eq!(compressed[0], 0x20);
        assert_eq!(compressed[1], 0x77);
        assert_eq!(compressed[2], 0x69);
    }
}
