/*
TODO
+ Fixed-length headers
+ Schema versioning
+ Optional fields using presence bits
+ Groups and repeating fields
+ Little-endian encoding, alignment, and padding
*/

pub mod encoder;
pub mod decoder;
pub mod types;


#[cfg(test)]
mod tests {
    use super::encoder::encode_order;
    use super::decoder::decode_order;
    use super::types::Order;

    #[test]
    fn test_order_encoding_roundtrip() {
        let original = Order {
            order_id: 42,
            quantity: 10,
            price: 123.45,
            version: 1,
        };

        let encoded = encode_order(&original);
        let decoded = decode_order(&encoded).unwrap();

        assert_eq!(original, decoded);
    }
}
