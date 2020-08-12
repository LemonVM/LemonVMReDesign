use super::{function::Function, TypeTags, io::{Reader, BinaryRW}};
use std::collections::BTreeMap;
// type tag + data
pub enum Constant{
    String(String),
    // object
    Object(BTreeMap<String,Constant>),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Function(Function),
    
    Opaque(Vec<u8>),
    // TODO: finish
    #[cfg(BIG_INT)]
    BigInt,
}

impl BinaryRW for Constant{
    fn read(reader:&mut Reader) -> Self {
        let tag = reader.read_u8();
        match tag{
            t if TypeTags::String as u8 == tag => {
                Constant::String(reader.read_string())
            }
            t if TypeTags::Object as u8 == tag => {
                Constant::Object(reader.read_map(|reader|{
                    (reader.read_string(),Constant::read(reader))
                }))
            }
            t if TypeTags::U8 as u8 == tag => {
                Constant::U8(reader.read_u8())
            }
            t if TypeTags::I8 as u8 == tag => {
                Constant::I8(reader.read_i8())
            }
            t if TypeTags::U16 as u8 == tag => {
                Constant::U16(reader.read_u16())
            }
            t if TypeTags::I16 as u8 == tag => {
                Constant::I16(reader.read_i16())
            }
            t if TypeTags::U32 as u8 == tag => {
                Constant::U32(reader.read_u32())
            }
            t if TypeTags::I32 as u8 == tag => {
                Constant::I32(reader.read_i32())
            }
            t if TypeTags::U64 as u8 == tag => {
                Constant::U64(reader.read_u64())
            }
            t if TypeTags::I64 as u8 == tag => {
                Constant::I64(reader.read_i64())
            }
            t if TypeTags::F32 as u8 == tag => {
                Constant::F32(reader.read_f32())
            }
            t if TypeTags::F64 as u8 == tag => {
                Constant::F64(reader.read_f64())
            }
            t if TypeTags::Opaque as u8 == tag => {
                Constant::Opaque(reader.read_vec(|reader|reader.read_u8()))
            }
            t if TypeTags::Function as u8 == tag => {
                Constant::Function(Function::read(reader))
            }
            _ => unimplemented!()
        }
    }
}