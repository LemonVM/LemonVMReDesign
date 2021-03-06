#[macro_use]
use std::collections::BTreeMap;

pub struct Reader {
    data: *const u8,
    pos: usize,
}

pub struct Writer {
    pub data: Vec<u8>,
}

pub trait BinaryRW {
    fn read(reader: &mut Reader) -> Self;
    fn write(&self, writer: &mut Writer);

    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>;
}

// #[cfg(mock)]
pub fn mock_string() -> String {
    use rand::*;
    let mut ret: String = String::new();
    for i in 0..10 {
        ret.push('草');
    }
    ret
}

#[macro_export]
macro_rules! gen_test_reader_writer_for_type {
    ($f:ident,$t:ident) => {
        #[test]
        fn $f() {
            let data = $t::mock_data();
            for d in data {
                let mut writer = Writer::new();
                d.write(&mut writer);
                let raw_data = writer.data.as_mut_ptr();
                let mut reader = Reader::new(raw_data);
                let read = $t::read(&mut reader);
                assert_eq!(*d, read)
            }
        }
    };
}

// #[cfg(mock)]
pub fn mock_object<K: Ord, V>(kf: &dyn Fn() -> K, vf: &dyn Fn() -> V) -> BTreeMap<K, V> {
    use rand::*;
    let mut ret: BTreeMap<K, V> = BTreeMap::new();
    for i in 0..10 {
        ret.insert(kf(), vf());
    }
    ret
}

impl Reader {
    pub fn new(data: *const u8) -> Reader {
        Reader { data, pos: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        unsafe {
            let b = *self.data.add(self.pos);
            self.pos += 1;
            b
        }
    }
    pub fn read_u16(&mut self) -> u16 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u16);
            self.pos += 2;
            b
        }
    }
    pub fn read_u32(&mut self) -> u32 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u32);
            self.pos += 4;
            b
        }
    }
    pub fn read_u64(&mut self) -> u64 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u64);
            self.pos += 8;
            b
        }
    }

    pub fn read_i8(&mut self) -> i8 {
        unsafe {
            let b = *self.data.add(self.pos);
            self.pos += 1;
            b as i8
        }
    }
    pub fn read_i16(&mut self) -> i16 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u16);
            self.pos += 2;
            b as i16
        }
    }
    pub fn read_i32(&mut self) -> i32 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u32);
            self.pos += 4;
            b as i32
        }
    }
    pub fn read_i64(&mut self) -> i64 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u64);
            self.pos += 8;
            b as i64
        }
    }

    pub fn read_f32(&mut self) -> f32 {
        unsafe {
            let i = *(self.data.add(self.pos) as *const u32);
            self.pos += 4;
            let bt = [i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8];
            f32::from_be_bytes(bt)
        }
    }
    pub fn read_f64(&mut self) -> f64 {
        unsafe {
            let i = *(self.data.add(self.pos) as *const u64);
            self.pos += 8;
            let bt = [
                i as u8,
                (i >> 8) as u8,
                (i >> 16) as u8,
                (i >> 24) as u8,
                (i >> 32) as u8,
                (i >> 40) as u8,
                (i >> 48) as u8,
                (i >> 56) as u8,
            ];
            f64::from_be_bytes(bt)
        }
    }

    pub fn read_string(&mut self) -> String {
        let vec = self.read_vec(|reader| reader.read_u8());
        String::from_utf8(vec).unwrap()
    }

    pub fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let n = self.read_u16() as usize;
        let mut vec = Vec::with_capacity(n);
        for _i in 0..n {
            vec.push(f(self));
        }
        vec
    }
    pub fn read_map<K, V, F>(&mut self, f: F) -> BTreeMap<K, V>
    where
        F: Fn(&mut Reader) -> (K, V),
        K: Ord,
    {
        let n = self.read_u16() as usize;
        let mut map = BTreeMap::new();
        for _i in 0..n {
            let (k, v) = f(self);
            map.insert(k, v);
        }
        map
    }
    pub fn read_option<T, F>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let flag = self.read_u8();
        if flag == 0x00 {
            None
        } else {
            Some(f(self))
        }
    }
}

impl Writer {
    pub fn new() -> Self {
        Writer { data: Vec::new() }
    }

    pub fn write_u8(&mut self, i: u8) {
        self.data.push(i);
    }

    pub fn write_u16(&mut self, i: u16) {
        self.data.append(&mut vec![i as u8, (i >> 8) as u8]);
    }

    pub fn write_u32(&mut self, i: u32) {
        self.data.append(&mut vec![
            i as u8,
            (i >> 8) as u8,
            (i >> 16) as u8,
            (i >> 24) as u8,
        ]);
    }

    pub fn write_u64(&mut self, i: u64) {
        self.data.append(&mut vec![
            i as u8,
            (i >> 8) as u8,
            (i >> 16) as u8,
            (i >> 24) as u8,
            (i >> 32) as u8,
            (i >> 40) as u8,
            (i >> 48) as u8,
            (i >> 56) as u8,
        ]);
    }

    pub fn write_f32(&mut self, i: f32) {
        let d = i.to_be_bytes();
        self.data.append(&mut d.into());
    }

    pub fn write_f64(&mut self, i: f64) {
        let d = i.to_be_bytes();
        self.data.append(&mut d.into());
    }

    pub fn write_string(&mut self, i: String) {
        let bytes = i.as_bytes();
        self.write_u16(bytes.len() as u16);
        for b in bytes {
            self.write_u8(*b);
        }
    }

    pub fn write_vec<T, F>(&mut self, inp: Vec<T>, f: F)
    where
        T: Clone,
        F: Fn(&mut Self, T),
    {
        self.write_u16(inp.len() as u16);
        for i in inp {
            f(self, i.clone());
        }
    }

    pub fn write_map<K, V, F>(&mut self, m: BTreeMap<K, V>, f: F)
    where
        K: Clone,
        V: Clone,
        K: Ord,
        F: Fn(&mut Self, (K, V)),
    {
        self.write_u16(m.len() as u16);
        for i in m {
            f(self, (i.0.clone(), i.1.clone()));
        }
    }

    pub fn write_option<T, F>(&mut self, o: Option<T>, f: F)
    where
        F: Fn(&mut Self, T),
    {
        match o {
            Some(x) => {
                self.write_u8(0x01);
                f(self, x);
            }
            _ => self.write_u8(0x00),
        };
    }
}
