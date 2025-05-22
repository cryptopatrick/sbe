use crate::types::Order;
use byteorder::{LittleEndian, WriteBytesExt};

pub fn encode_order(order: &Order) -> Vec<u8> {
    let mut buf = Vec::with_capacity(22); // 8 + 4 + 8 + 2
    buf.write_u64::<LittleEndian>(order.order_id).unwrap();
    buf.write_u32::<LittleEndian>(order.quantity).unwrap();
    buf.write_f64::<LittleEndian>(order.price).unwrap();
    buf.write_u16::<LittleEndian>(order.version).unwrap();

    buf
}
