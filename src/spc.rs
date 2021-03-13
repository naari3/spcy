use id666_rs::ID666;
use snes_spc::{SNESSpc, SpcFilter};

use anyhow::Result;

pub struct Spc {
    innser_spc: SNESSpc,
    inner_filter: SpcFilter,
    total_frames: u32,
    remain_frames: u32,
}

unsafe impl Send for Spc {}

impl Spc {
    pub fn from(data: &mut [u8]) -> Result<Self> {
        let id6 = ID666::from(data).unwrap();
        Ok(Self {
            innser_spc: SNESSpc::from(data)?,
            inner_filter: SpcFilter::new(),
            total_frames: id6.total_len.unwrap(),
            remain_frames: id6.total_len.unwrap(),
        })
    }

    pub fn samples(&mut self) -> Result<Vec<i16>> {
        let mut buf = [0i16; 2 * 4096];
        let mut result: Vec<i16> = Vec::with_capacity((self.total_frames * 2) as usize);

        while self.remain_frames != 0 {
            let fc = if self.remain_frames < 4096 {
                self.remain_frames
            } else {
                4096
            };
            let stereo_count = (fc * 2) as i32;
            self.innser_spc.play(stereo_count, &mut buf)?;
            self.inner_filter.run(&mut buf, stereo_count);

            result.append(&mut buf.to_vec());

            self.remain_frames -= fc;
        }

        Ok(result)
    }
}

pub fn spc_to_samples(data: &mut [u8]) -> Result<Vec<i16>> {
    let mut spc = Spc::from(data)?;
    Ok(spc.samples()?)
}
