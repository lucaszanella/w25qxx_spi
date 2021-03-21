use w25qlib::*;
use std::ffi::CString;

const SPI_BW: u8 = 8;
const SPI_DELAY: u16 = 0;
const SR1_BUSY_MASK: u8 = 0x01;
const CMD_WRIRE_ENABLE: u8 = 0x06;
const CMD_READ_STATUS_R1: u8 = 0x05;
const CMD_READ_UNIQUE_ID: u8 = 0x4B;
const CMD_READ_STATUS_R2: u8 = 0x35;
const CMD_READ_STATUS_R3: u8 = 0x11;
const CMD_SECTOR_ERASE: u8 = 0x20;
const CMD_MANUFACURER_ID: u8 = 0x90;
const CMD_JEDEC_ID: u8 = 0x9f;
const CMD_PAGE_PROGRAM: u8 = 0x02;
const CMD_READ_DATA: u8 = 0x03;

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

    pub fn spi_data_rw(&mut self, channel_: i32, data: &mut [u8]) -> i32 {
        let channel = channel_ & 1;
        let mut spi = SpiIocTransfer::new();
        spi.tx_buf        = data.as_mut_ptr() as u64;//(unsigned long) ?
        spi.rx_buf        = data.as_mut_ptr() as u64;//(unsigned long) ?
        spi.len           = data.len() as u32;
        spi.delay_usecs   = SPI_DELAY;
        spi.speed_hz      = self.spi_speeds[channel as usize] as u32;
        spi.bits_per_word = SPI_BW;
        panic!("to be implemented");
        //libc::ioctl(self.spi_fds[channel as usize], ioctls::SPI_IOC_MESSAGE(1), &spi)
    }

    fn spi_setup_mode(&mut self, channel: i32, speed: u32, mode_: i32) -> Result<i32,String>{
        let mut fd: i32 = -1;
        let spi_dev:[::std::os::raw::c_char; 32] = [0; 32];
        let mode = mode_ & 3;
        let s = CString::new("/dev/spidev0.%d").expect("CString::new failed");
        panic!("to be implemented");

        unsafe{libc::snprintf(spi_dev.as_mut_ptr(), 31, s.as_ptr(), channel)};
        fd = unsafe{libc::open (spi_dev.as_mut_ptr(), libc::O_RDWR)};

        if fd < 0 {
            return Err(format!("Unable to open SPI device: {}", 1));
        }

        self.spi_speeds[channel as usize] = speed;
        self.spi_fds[channel as usize] = fd;
        //libc::ioctl(fd, ioctls::SPI_IOC_WR_MODE, &mode);
        //unsafe{ioctls::spi_ioc_wr_mode(fd, &mode)};
        if fd < 0 {
            return Err(format!("SPI Mode Change failure, {}", 1));
        }

        //libc::ioctl(fd, ioctls::SPI_IOC_WR_BITS_PER_WORD, SPI_BW as libc::c_uint);
        //unsafe{ioctls::spi_ioc_wr_mode(fd, SPI_BW as libc::c_uint)};
        if fd < 0 {
            return Err(format!("SPI BPW Change failure: {}", 1));
        }

        //libc::ioctl(fd, ioctls::SPI_IOC_WR_MAX_SPEED_HZ, &speed);
        //ioctls::spi_ioc_wr_max_speed_hz(fd, &speed);
        if fd < 0 {
            return Err(format!("SPI Speed Change failure: {}", 1));
        }

        Ok(fd)
    }

    pub fn read_status_register_1(&self) -> Result<[u8;2], i32>{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = CMD_READ_STATUS_R1;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read_status_register_2(&self) ->  Result<[u8;2], i32>{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = CMD_READ_STATUS_R2;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read_status_register_3(&self) -> Result<[u8;2], i32>{
        let mut slice :[u8;2] = [0;2];
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = CMD_READ_STATUS_R3;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data);
        Ok(slice)
    }

    pub fn read_unique_id(&self) ->  Result<[u8;8], i32> {
        let mut slice :[u8;8] = [0;8];
        let mut data: [::std::os::raw::c_char; 13] = [0;13];
        data[0] = CMD_READ_UNIQUE_ID;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data[5..]);
        Ok(slice)
    }

    pub fn read(&self, address: u32, number_of_bytes: u16) ->  Result<Vec<u8>, u16> {
        let s: usize = number_of_bytes as usize + 4;
        let mut data = vec![0u8;s];
        data[0] = CMD_READ_DATA;
        data[1] = (address>>16 & 0xFF) as u8;     // A23-A16
        data[2] = (address>>8 & 0xFF) as u8;      // A15-A08
        data[3] = (address & 0xFF) as u8;         // A07-A00
        /*
        for i in 1..4 {
            println!("read[{}] = {}", i, data[i]);
        }
        */
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_slice().as_mut_ptr(), data.len() as i32)};
        let v = (&(data.as_slice())[4..]).to_vec();
        Ok(v)
    }

    pub fn is_busy(&mut self) -> bool{
        let mut data: [::std::os::raw::c_char; 2] = [0;2];
        data[0] = CMD_READ_STATUS_R1;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        let mut r1: u8 = 0;
        r1 = data[1];
        if (r1 & SR1_BUSY_MASK) !=0 {
            return true;
        }
        false
    }

    pub fn write_enable(&mut self) {
        let mut data: [::std::os::raw::c_char; 1] = [0;1];
        data[0] = CMD_WRIRE_ENABLE;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
    }

    pub fn page_write(&mut self, sector_number: u16, address: u32, buffer: &[u8]) -> u32 {
        if (buffer.len() > 256) {
            return 0;
        }
        let mut _r: i32 = 0;
        /*
        let mut address:u32 = sector_number as u32;
        address <<= 12;
        address += input_address as u32;
        */
        self.write_enable();
        if self.is_busy() {
            return 0;
        }
        let mut data = vec![0u8;(buffer.len()+4) as usize];
        data[0] = CMD_PAGE_PROGRAM;
        data[1] = ((address>>16) & 0xFF) as u8;
        data[2] = ((address>>8) & 0xFF) as u8;
        data[3] = (address & 0xFF) as u8;
        &data[4..].copy_from_slice(buffer);
        /*
        println!("write dump:");
        for i in 0..data.len() {
            print!("{},", data[i as usize]);
        }
        println!("");
        */
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_slice().as_mut_ptr(), (buffer.len() + 4) as i32)};
        loop {
            if !self.is_busy() {
                break;
            }
        }
        (_r - 4) as u32
    }

    pub fn erase_sector(&mut self, sector_number: u16, figwait: bool) -> bool {
        let mut slice :[u8;4] = [0;4];
        let mut data: [::std::os::raw::c_char; 4] = [0;4];
        let mut address:u32 = sector_number as u32;
        address <<= 12;
        self.write_enable();
        data[0] = CMD_SECTOR_ERASE;
        data[1] = ((address>>16) & 0xff) as u8;
        data[2] = ((address>>8) & 0xff) as u8;
        data[3] = (address & 0xff) as u8;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        loop {
            if (self.is_busy() && figwait) {
                std::thread::sleep(std::time::Duration::from_millis(10));
            } else {
                break;
            }
        }
        true
    }

    pub fn erase_address(&mut self, address: u32, figwait: bool) -> bool {
        let mut slice :[u8;4] = [0;4];
        let mut data: [::std::os::raw::c_char; 4] = [0;4];
        self.write_enable();
        data[0] = CMD_SECTOR_ERASE;
        data[1] = ((address>>16) & 0xff) as u8;
        data[2] = ((address>>8) & 0xff) as u8;
        data[3] = (address & 0xff) as u8;
        let mut _r: i32 = 0;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel,data.as_mut_ptr(), data.len() as i32)};
        loop {
            if (self.is_busy() && figwait) {
                std::thread::sleep(std::time::Duration::from_millis(10));
            } else {
                break;
            }
        }
        true
    }

    pub fn read_manufacturer_id(&self) -> Result<[u8; 6], i32> {
        let mut slice :[u8;6] = [0;6];
        let mut data: [::std::os::raw::c_char; 6] = [0;6];
        let mut _r: i32 = 0;
        data[0] = CMD_MANUFACURER_ID;
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
        data[0] = CMD_JEDEC_ID;
        _r = unsafe{wiringPiSPIDataRW(self.spi_channel, data.as_mut_ptr(), data.len() as i32)};
        slice.clone_from_slice(&data[0..]);
        Ok(slice)
    }
}