use id666::ID666;
use snes_spc::{SNESSpc, SpcFilter};

use anyhow::Result;

pub struct Spc {
    innser_spc: SNESSpc,
    inner_filter: SpcFilter,
    total_frames: u32,
    remain_frames: u32,
    fade_frames: u32,
}

unsafe impl Send for Spc {}

impl Spc {
    pub fn from(data: &mut [u8]) -> Result<Self> {
        let id6 = ID666::from(data)?;
        let total_len = match id6.total_len {
            Some(i) => i,
            None => {
                panic!("Maybe corrupt result")
            }
        };
        let fade_frames = id6.fade.unwrap_or(0);
        Ok(Self {
            innser_spc: SNESSpc::from(data)?,
            inner_filter: SpcFilter::new(),
            total_frames: total_len,
            remain_frames: total_len,
            fade_frames: fade_frames,
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
            fade_frames(&mut buf, self.remain_frames, self.fade_frames, fc);

            result.append(&mut buf.to_vec());

            self.remain_frames -= fc;
        }

        Ok(result)
    }
}

fn fade_frames(io: &mut [i16], remain_frames: u32, fade_frames: u32, count: u32) {
    let mut i = 0;
    let mut f = fade_frames;

    if remain_frames - count > fade_frames {
        return;
    }
    if remain_frames > fade_frames {
        i = remain_frames - fade_frames;
        f += i;
    } else {
        f = remain_frames;
    }

    while i < count {
        let fade = (f - i) as f64 / fade_frames as f64;
        io[((i * 2) + 0) as usize] = (fade * (io[((i * 2) + 0) as usize] as f64)) as i16;
        io[((i * 2) + 1) as usize] = (fade * (io[((i * 2) + 1) as usize] as f64)) as i16;
        i += 1;
    }
}

pub fn spc_to_samples(data: &mut [u8]) -> Result<Vec<i16>> {
    let mut spc = Spc::from(data)?;
    Ok(spc.samples()?)
}
