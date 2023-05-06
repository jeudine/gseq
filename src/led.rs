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
		let phase = phase.lock().unwrap();
		let cmd = if phase.gains[0] > 1.0 {
			MpsseCmdBuilder::new().set_gpio_lower(0xFF, 0xFF)
		} else {
			MpsseCmdBuilder::new().set_gpio_lower(0x0, 0xFF)
		};
		ft232h.write_all(cmd.as_slice()).unwrap();
	});
	Ok(())
}
