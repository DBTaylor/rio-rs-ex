//generated with nifpga-apigen
use nifpga::{NifpgaError, Session, ReadFifo, WriteFifo};

pub struct Fpga {
	pub session: Session
}

impl Fpga {
	pub fn open() -> Result<Fpga, NifpgaError>{
		Ok(Fpga{session: Session::open(
			"/home/lvuser/fpga.lvbitx",
			"7100ED41CF70D00C35169EC9CB8882EA",
			"RIO0",
			true,
			true
		)?})
	}
	pub fn read_bool_out_0(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x1802A)
	}
	pub fn read_bool_out_1(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x18026)
	}
	pub fn read_bool_out_2(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x18022)
	}
	pub fn read_bool_out_3(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x1801E)
	}
	pub fn read_bool_out_4(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x1801A)
	}
	pub fn read_bool_out_5(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x18016)
	}
	pub fn read_bool_out_6(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x18012)
	}
	pub fn read_bool_out_7(&self) -> Result<bool, NifpgaError>{
		self.session.read::<bool>(0x1800E)
	}
	pub fn read_indicator_0(&self) -> Result<f32, NifpgaError>{
		self.session.read::<f32>(0x1805C)
	}
	pub fn read_indicator_1(&self) -> Result<f32, NifpgaError>{
		self.session.read::<f32>(0x1804C)
	}
	pub fn write_bool_0(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x1804A, value)
	}
	pub fn write_bool_1(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x18046, value)
	}
	pub fn write_bool_2(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x18042, value)
	}
	pub fn write_bool_3(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x1803E, value)
	}
	pub fn write_bool_4(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x1803A, value)
	}
	pub fn write_bool_5(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x18036, value)
	}
	pub fn write_bool_6(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x18032, value)
	}
	pub fn write_bool_7(&self, value: bool) -> Result<(), NifpgaError>{
		self.session.write(0x1802E, value)
	}
	pub fn write_control_0(&self, value: f32) -> Result<(), NifpgaError>{
		self.session.write(0x18054, value)
	}
	pub fn write_control_1(&self, value: f32) -> Result<(), NifpgaError>{
		self.session.write(0x18000, value)
	}
	pub fn read_bool_arr_out(&self) -> Result<[bool; 8], NifpgaError>{
		let mut array: [bool; 8] = Default::default();
		self.session.read_array::<bool>(0x18006, &mut array)?;
		Ok(array)
	}
	pub fn read_arr_indicator(&self) -> Result<[f32; 2], NifpgaError>{
		let mut array: [f32; 2] = Default::default();
		self.session.read_array::<f32>(0x18058, &mut array)?;
		Ok(array)
	}
	pub fn write_bool_arr(&self, array: &[bool; 8]) -> Result<(), NifpgaError>{
		self.session.write_array::<bool>(0x1800A, array)
	}
	pub fn write_arr_control(&self, array: &[f32; 2]) -> Result<(), NifpgaError>{
		self.session.write_array::<f32>(0x18050, array)
	}
	pub fn read_bool_outs(&self) -> Result<[bool; 8], NifpgaError>{
		let mut array: [bool; 8] = Default::default();
		array[0] = self.session.read::<bool>(0x1802A)?;
		array[1] = self.session.read::<bool>(0x18026)?;
		array[2] = self.session.read::<bool>(0x18022)?;
		array[3] = self.session.read::<bool>(0x1801E)?;
		array[4] = self.session.read::<bool>(0x1801A)?;
		array[5] = self.session.read::<bool>(0x18016)?;
		array[6] = self.session.read::<bool>(0x18012)?;
		array[7] = self.session.read::<bool>(0x1800E)?;
		Ok(array)
	}
	pub fn read_indicators(&self) -> Result<[f32; 2], NifpgaError>{
		let mut array: [f32; 2] = Default::default();
		array[0] = self.session.read::<f32>(0x1805C)?;
		array[1] = self.session.read::<f32>(0x1804C)?;
		Ok(array)
	}
	pub fn write_bools(&self, array: &[bool; 8]) -> Result<(), NifpgaError>{
		self.session.write(0x1804A, array[0])?;
		self.session.write(0x18046, array[1])?;
		self.session.write(0x18042, array[2])?;
		self.session.write(0x1803E, array[3])?;
		self.session.write(0x1803A, array[4])?;
		self.session.write(0x18036, array[5])?;
		self.session.write(0x18032, array[6])?;
		self.session.write(0x1802E, array[7])?;
		Ok(())
	}
	pub fn write_controls(&self, array: &[f32; 2]) -> Result<(), NifpgaError>{
		self.session.write(0x18054, array[0])?;
		self.session.write(0x18000, array[1])?;
		Ok(())
	}
	pub fn open_target_to_host(&self, depth: usize) -> Result<(ReadFifo<f32>, usize), NifpgaError>{
		self.session.open_read_fifo::<f32>(0, depth)
	}
	pub fn open_host_to_target(&self, depth: usize) -> Result<(WriteFifo<f32>, usize), NifpgaError>{
		self.session.open_write_fifo::<f32>(1, depth)
	}
}