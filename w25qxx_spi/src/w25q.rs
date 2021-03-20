use wiringpi::*;
use std::ffi::CString;

const CMD_JEDEC_ID: u8 = 0x9f;
const SPI_BW: u8 = 8;
const SPI_DELAY: u16 = 0;

pub struct W25Q {
    spi_channel: i32,
    spi_speed: i32,
    spi_speeds: [u32; 2],
    spi_fds: [i32; 2]
}

struct SpiIocTransfer {
    tx_buf: u64,
    rx_buf: u64,
    len: u32,
    speed_hz: u32,
    delay_usecs: u16,
    bits_per_word: u8,
    cs_change: u8,
    pad: u32
    /* If the contents of 'struct spi_ioc_transfer' ever change
    * incompatibly, then the ioctl number (currently 0) must change;
    * ioctls with constant size fields get a bit more in the way of
    * error checking than ones (like this) where that field varies.
    *
    * NOTE: struct layout is the same in 64bit and 32bit userspace.
    */
}

impl SpiIocTransfer {
    pub fn new() -> SpiIocTransfer {
        SpiIocTransfer{
            tx_buf: 0,
            rx_buf: 0,
            len: 0,
            speed_hz: 0,
            delay_usecs: 0,
            bits_per_word: 0,
            cs_change: 0,
            pad: 0
        }
    }
}

impl W25Q {
    pub fn new(spi_channel: i32, spi_speed: i32) -> Result<W25Q, ()> {
        let w = W25Q {
            spi_channel: spi_channel,
            spi_speed: spi_speed,
            spi_speeds: [0,0],
            spi_fds: [0,0]
        };
        let r = unsafe{wiringPiSPISetup(w.spi_channel, w.spi_speed)};
        if r>=0 {
            Ok(w)
        } else {
            Err(())
        }
    }

    pub fn spi_data_rw(&mut self, channel_: i32, data: &[u8], len: u32) -> i32 {
        let channel = channel_ & 1;
        let spi = SpiIocTransfer::new();
        spi.tx_buf        = data.as_mut_ptr();//(unsigned long) ?
        spi.rx_buf        = data.as_mut_ptr();//(unsigned long) ?
        spi.len           = len;
        spi.delay_usecs   = SPI_DELAY;
        spi.speed_hz      = self.spi_speeds[channel as usize] as u32;
        spi.bits_per_word = SPI_BW;
        
        libc::ioctl(self.spi_fds[channel as usize], ioctls::SPI_IOC_MESSAGE(1), &spi)
    }

    /*
        int wiringPiSPIDataRW (int channel, unsigned char *data, int len)
        {
        struct spi_ioc_transfer spi ;

        channel &= 1 ;

        // Mentioned in spidev.h but not used in the original kernel documentation
        //	test program )-:

        memset (&spi, 0, sizeof (spi)) ;

        spi.tx_buf        = (unsigned long)data ;
        spi.rx_buf        = (unsigned long)data ;
        spi.len           = len ;
        spi.delay_usecs   = spiDelay ;
        spi.speed_hz      = spiSpeeds [channel] ;
        spi.bits_per_word = spiBPW ;

        return ioctl (spiFds [channel], SPI_IOC_MESSAGE(1), &spi) ;
        }
    */

    fn spi_setup_mode(&mut self, channel: i32, speed: u32, mode_: i32) -> Result<i32,String>{
        let mut fd: i32 = -1;
        let mut spi_dev:[::std::os::raw::c_char; 32] = [0; 32];
        let mode = mode_ & 3;
        let s = CString::new("/dev/spidev0.%d").expect("CString::new failed");
        unsafe{libc::snprintf(spi_dev.as_mut_ptr(), 31, s.as_ptr(), channel)};
        fd = libc::open (spi_dev.as_mut_ptr(), libc::O_RDWR);

        if fd < 0 {
            return Err(format!("Unable to open SPI device: {}", 1));
        }

        self.spi_speeds[channel as usize] = speed;
        self.spi_fds[channel as usize] = fd;
        //libc::ioctl(fd, ioctls::SPI_IOC_WR_MODE, &mode);
        unsafe{ioctls::spi_ioc_wr_mode(fd, &mode)};
        if fd < 0 {
            return Err(format!("SPI Mode Change failure, {}", 1));
        }

        //libc::ioctl(fd, ioctls::SPI_IOC_WR_BITS_PER_WORD, SPI_BW as libc::c_uint);
        unsafe{ioctls::spi_ioc_wr_mode(fd, SPI_BW as libc::c_uint)};
        if fd < 0 {
            return Err(format!("SPI BPW Change failure: {}", 1));
        }

        //libc::ioctl(fd, ioctls::SPI_IOC_WR_MAX_SPEED_HZ, &speed);
        ioctls::spi_ioc_wr_max_speed_hz(fd, &speed);
        if fd < 0 {
            return Err(format!("SPI Speed Change failure: {}", 1));
        }

        Ok(fd)
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
        let mut data = vec![0u8;s];
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