use crate::fft;
use ftdi_mpsse::MpsseCmdBuilder;
use libftd2xx::{Ft232h, Ftdi, FtdiCommon, FtdiMpsse};
use std::thread;

pub fn init(levels: &fft::Levels) -> Result<(), Box<dyn std::error::Error>> {
	let mut ft232h: Ft232h = Ftdi::new()?.try_into()?;
	ft232h.initialize_mpsse_default()?;
	let cmd = MpsseCmdBuilder::new().set_gpio_lower(0xFF, 0xFF);
	ft232h.write_all(cmd.as_slice())?;
	let levels = levels.clone();
	thread::spawn(move || loop {
		let gain: Vec<_> = {
			let level = levels.lock().unwrap();
			level.iter().map(|x| (x.val - x.mean) / x.sd).collect()
		};
		let cmd = if gain[0] > 1.0 {
			MpsseCmdBuilder::new().set_gpio_lower(0xFF, 0xFF)
		} else {
			MpsseCmdBuilder::new().set_gpio_lower(0x0, 0xFF)
		};
		ft232h.write_all(cmd.as_slice()).unwrap();
	});
	Ok(())
}
