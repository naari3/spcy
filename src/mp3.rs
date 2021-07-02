use core::panic;

use anyhow::Result;
use lame;

pub fn samples_to_mp3(samples: &mut [i16]) -> Result<Vec<u8>> {
    let mut lame = match lame::Lame::new() {
        Some(lame) => lame,
        None => {
            panic!("Failed to initialize lame instance");
        }
    };
    lame.set_sample_rate(32000)?;
    lame.set_channels(2)?;
    lame.set_kilobitrate(92)?;
    lame.init_params()?;

    let mut buf = vec![0u8; samples.len() * 4];

    let mut pcm_left = vec![];
    let mut pcm_right = vec![];

    for (i, &sample) in samples.iter().enumerate() {
        if i % 2 == 0 {
            pcm_left.push(sample);
        } else {
            pcm_right.push(sample);
        }
    }

    let size = lame.encode(&mut pcm_left, &mut pcm_right, &mut buf)?;

    let buf = &buf[..size];
    Ok(buf.to_owned())
}
