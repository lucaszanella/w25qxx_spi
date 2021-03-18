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

    pub fn read_status_register_1(&self) -> [u8;2]{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = 0x05;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW (self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        slice
    }


    /*
    //
    // ステータスレジスタ1の値取得
    // 戻り値: ステータスレジスタ1の値
    //
    uint8_t W25Q64_readStatusReg1(void) {
    unsigned char data[2];
    int rc;
    UNUSED(rc);
    data[0] = CMD_READ_STATUS_R1;
    rc = wiringPiSPIDataRW (_spich,data,sizeof(data));
    //spcDump("readStatusReg1",rc,data,2);
    return data[1];
    }

    //
    // ステータスレジスタ2の値取得
    // 戻り値: ステータスレジスタ2の値
    //
    uint8_t W25Q64_readStatusReg2(void) {
    unsigned char data[2];
    int rc;
    UNUSED(rc);
    data[0] = CMD_READ_STATUS_R2;
    rc = wiringPiSPIDataRW (_spich,data,sizeof(data));
    //spcDump("readStatusReg2",rc,data,2);
    return data[1];
    }
    */
    pub fn read_manufacturer_id(&self) -> [u8; 6] {
        let mut slice :[u8;6] = [0;6];
        let mut data: [::std::os::raw::c_char; 6] = [0;6];
        let mut _r: i32 = 0;
        data[0] = 0x90;
        data[3] = 0x00;
        data[4] = 0xEF;
        data[5] = 0x17;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data[0..]);
        slice
    }

    pub fn read_jedec_id(&self) -> [u8; 4] {
        let mut slice :[u8;4] = [0;4];
        let mut data: [::std::os::raw::c_char; 4] = [0;4];
        let mut _r: i32 = 0;
        data[0] = 0x9F;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data[0..]);
        slice
    }
}