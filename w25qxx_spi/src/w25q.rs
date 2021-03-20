use wiringpi::*;

const CMD_JEDEC_ID: u8 = 0x9f;


pub struct W25Q {
    spi_channel: i32,
    spi_speed: i32
}

impl W25Q {
    pub fn new(spi_channel: i32, spi_speed: i32) -> Result<W25Q, ()> {
        let w = W25Q {
            spi_channel: spi_channel,
            spi_speed: spi_speed,
        };
        let r = unsafe{wiringPiSPISetup(w.spi_channel, w.spi_speed)};
        if r>=0 {
            Ok(w)
        } else {
            Err(())
        }
    }

    pub fn read_status_register_1(&self) -> Result<[u8;2], i32>{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = 0x05;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW (self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read_status_register_2(&self) ->  Result<[u8;2], i32>{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = 0x35;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW (self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read_status_register_3(&self) -> Result<[u8;2], i32>{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = 0x11;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW (self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read_unique_id(&self) ->  Result<[u8;13], i32> {
        let mut slice :[u8;13] = [0;13];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = 0x4B;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW (self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read(&self, address: u32, number_of_bytes: u16) ->  Result<Vec<u8>, u16> {
        let s: usize = number_of_bytes as usize + 4;
        let mut data = Vec::<::std::os::raw::c_char>::with_capacity(s);
        data.set_len(s);
        data[0] = 0x4B;
        data[1] = ((address>>16) & 0xFF) as u8;     // A23-A16
        data[2] = ((address>>8) & 0xFF) as u8;      // A15-A08
        data[3] = (address & 0xFF) as u8;           // A07-A00
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_slice().as_mut_ptr(), data.len() as i32)};
        let v = (&(data.as_slice())[4..]).to_vec();
        Ok(v)
    }

    pub fn read_manufacturer_id(&self) -> Result<[u8; 6], i32> {
        let mut slice :[u8;6] = [0;6];
        let mut data: [::std::os::raw::c_char; 6] = [0;6];
        let mut _r: i32 = 0;
        data[0] = 0x90;
        data[3] = 0x00;
        data[4] = 0xEF;
        data[5] = 0x17;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data[0..]);
        Ok(slice)
    }

    pub fn read_jedec_id(&self) -> Result<[u8; 4], i32> {
        let mut slice :[u8;4] = [0;4];
        let mut data: [::std::os::raw::c_char; 4] = [0;4];
        let mut _r: i32 = 0;
        data[0] = 0x9F;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data[0..]);
        Ok(slice)
    }
}