use ethereum_types::{H160, U256};

pub fn convert_u256_to_eth_address(value: U256) -> H160 {
    let mut be_u256 = [0u8; 32];
    value.to_big_endian(&mut be_u256);
    H160::from_slice(&be_u256[12..])
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn convert_u256_as_address() {
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
}
