use crate::types::Order;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub fn decode_order(data: &[u8]) -> Option<Order> {
    if data.len() < 22 {
        return None;
    }

    let mut rdr = Cursor::new(data);
    let order_id = rdr.read_u64::<LittleEndian>().ok()?;
    let quantity = rdr.read_u32::<LittleEndian>().ok()?;
    let price = rdr.read_f64::<LittleEndian>().ok()?;
    let version = rdr.read_u16::<LittleEndian>().ok()?;

    Some(Order {
        order_id,
        quantity,
        price,
        version,
    })
}
