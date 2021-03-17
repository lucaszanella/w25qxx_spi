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

    pub fn read_manufacturer_id(&self) -> [u8; 6] {
        let mut slice :[u8;3] = [0;3];
        let mut data: [::std::os::raw::c_char; 6] = [0;6];
        let mut _r: i32 = 0;
        data[0] = 0x90;
        data[3] = 0x00;
        data[4] = 0xEF;
        data[5] = 0x17;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), 6)};
        slice.clone_from_slice(&data[0..]);
        slice
    }

    pub fn read_jedec_id(&self) -> [u8; 4] {
        let mut slice :[u8;3] = [0;3];
        let mut data: [::std::os::raw::c_char; 4] = [0;4];
        let mut _r: i32 = 0;
        data[0] = 0x9F;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), 4)};
        slice.clone_from_slice(&data[0..]);
        slice
    }
}