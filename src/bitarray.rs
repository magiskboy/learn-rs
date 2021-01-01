use std::option::Option;

pub struct Bitarray {
    page: Vec<u8>,
    len: usize,
}

impl Bitarray {
    pub fn new(len: usize) -> Bitarray {
        let num_page = ((len as f32) / 8.0).ceil() as usize;
        Bitarray {
            page: vec![0; num_page],
            len,
        }
    }

    pub fn set(&mut self, pos: usize, value: bool) -> bool {
        assert!(pos < self.len, "The position is invalid");

        let page_id = (((pos as f32) / 8.0).ceil() - 1.0) as usize;
        let page = self.page.get(page_id);
        match page {
            Option::None => {
                false
            },
            Option::Some(v) => {
                let pos_id = pos % 8;
                let mut _v: u8;
                // on bit
                if value {
                    let mask = (1 << pos_id) as u8;
                    _v = v | &mask;
                }
                // off bit
                else {
                    let mask = !((1 << pos_id) as u8);
                    _v = v & &mask;
                }
                self.page[page_id] = _v;
                true
            }
        }
    }

    pub fn on(&mut self, pos: usize) -> bool {
        self.set(pos, true)
    }

    pub fn off(&mut self, pos: usize) -> bool {
        self.set(pos, false)
    }
}
