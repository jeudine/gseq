use crate::fft;
use ftdi_mpsse::MpsseCmdBuilder;
use libftd2xx::{Ft232h, Ftdi, FtdiCommon, FtdiMpsse};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn init(phase: &Arc<Mutex<fft::Phase>>) -> Result<(), Box<dyn std::error::Error>> {
	let mut ft232h: Ft232h = Ftdi::new()?.try_into()?;
	ft232h.initialize_mpsse_default()?;
	let cmd = MpsseCmdBuilder::new().set_gpio_lower(0xFF, 0xFF);
	ft232h.write_all(cmd.as_slice())?;
	let phase = phase.clone();
	thread::spawn(move || loop {
		let phase = {
			let p = phase.lock().unwrap();
			p.clone()
		};
		let mut val = 0;
		match phase.state {
			fft::State::Break(b) => {
				if phase.gains[0] > 3.0 {
					val |= 0x0F;
				}
				match b {
					fft::Break::State0 => {
						if phase.gains[2] > 0.5 {
							val |= 0x08;
						}
						if phase.gains[3] > 0.5 {
							val |= 0x01;
						}
					}
					fft::Break::State1 => {
						if phase.gains[2] > 1.0 {
							val |= 0x04;
						}
						if phase.gains[3] > 2.0 {
							val |= 0x08;
						}
					}
					fft::Break::State2 => {
						if phase.gains[3] > 3.0 {
							val |= 0x02;
						}
						if phase.gains[2] > 0.5 {
							val |= 0x08;
						}
					}
					fft::Break::State3 => {
						if phase.gains[3] > 0.5 {
							val |= 0x03;
						}
						if phase.gains[3] > 4.0 {
							val |= 0x0A;
						}
					}
				}
			}
			fft::State::Drop(d) => match d {
				fft::Drop::State0 => {
					if phase.gains[0] > 0.5 {
						val |= 0x03;
					}
					if phase.gains[3] > 0.5 {
						val |= 0x0C;
					}
				}
				fft::Drop::State1 => {
					if phase.gains[0] > 0.5 {
						val |= 0x03;
					}
					if phase.gains[2] > 0.5 {
						val |= 0x0C;
					}
				}
				fft::Drop::State2 => {
					if phase.gains[0] > 0.5 {
						val |= 0x03;
					}
					if phase.gains[2] > 0.5 {
						val |= 0x04;
					}
					if phase.gains[3] > 0.5 {
						val |= 0x08;
					}
				}
				fft::Drop::State3 => {
					if phase.gains[0] > 0.5 {
						val |= 0x0C;
					}
					if phase.gains[2] > 0.5 {
						val |= 0x01;
					}
					if phase.gains[3] > 0.5 {
						val |= 0x02;
					}
				}
			},
		};
		let cmd = MpsseCmdBuilder::new().set_gpio_lower(val, 0xFF);
		ft232h.write_all(cmd.as_slice()).unwrap();
	});
	Ok(())
}
