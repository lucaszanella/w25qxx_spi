mod w25q;
use w25q::W25Q;
use sha2::{Sha256, Digest};
use rand::Rng; // 0.8.0

const SPI_CHANNEL: i32 = 0;
//2MHz
const SPEED: i32 = 2000000;

fn dump_slice(slice: &[u8]) {
    for i in 0..slice.len() {
        if i!=slice.len()-1 {
            print!("{},", slice[i]);
        } else { 
            print!("{}", slice[i]);
        }
    }
    println!("");
}

fn dump_vec(vec: &Vec<u8>) {
    for i in 0..vec.len() {
        if i!=vec.len()-1 {
            print!("{},", vec[i]);
        } else { 
            print!("{}", vec[i]);
        }
    }
    println!("");
}

fn dump_vec_subset(vec: &Vec<u8>, start: usize) {
    for i in 0..vec.len() {
        if i!=vec.len()-1 {
            print!("{},", vec[i + start]);
        } else { 
            print!("{}", vec[i + start]);
        }
    }
    println!("");
}

fn dump_hash(array: &[u8]) {
    for i in 0..array.len() {
        print!("{}", array[i]);
    }
    println!("");
}

fn main() {

    let mut w25q = W25Q::new(SPI_CHANNEL, SPEED).unwrap();
    let register_1 = w25q.read_status_register_1().unwrap();
    let register_2 = w25q.read_status_register_2().unwrap();
    let register_3 = w25q.read_status_register_3().unwrap();
    let manufacturer_id = w25q.read_manufacturer_id().unwrap();
    let jedec_id = w25q.read_jedec_id().unwrap();
    let unique_id = w25q.read_unique_id().unwrap();

    print!("manufacturer_id: ");
    dump_slice(&manufacturer_id);
    print!("jedec_id: ");
    dump_slice(&jedec_id);
    print!("unique_id:: ");
    dump_slice(&unique_id);
    print!("register_1: ");
    dump_slice(&register_1);
    print!("register_2: ");
    dump_slice(&register_2);
    print!("register_3: ");
    dump_slice(&register_3);

    
    let mut hasher = Sha256::new();
    //16mb or 128mbit
    let base2: u32 = 2;
    let total_size: u32 = base2.pow(24);
    let mut data_to_write = vec![0u8;total_size as usize];
    println!("randomizing data...");
    for i in 0..data_to_write.len() {
        data_to_write[i as usize] =  rand::thread_rng().gen_range(0..255);
    }
    println!("calculating sha256sum of data_to_write from RAM (not spi)");
    hasher.update(&data_to_write.as_slice());
    let result = hasher.finalize();

    let per_write: u16 = 256;
    let s:u32 = total_size/(per_write as u32);
    let mut bytes_written = 0;
    println!("writing to spi...");
    for i in 0..s {
        let begin: usize = s as usize;
        let end: usize = s as usize + per_write as usize;
        w25q.erase_address(s, true);
        let n = w25q.page_write(0, s, &data_to_write.as_slice()[begin..end]);
        bytes_written += n;
    }
    println!("bytes written: {}", bytes_written);
    println!("sha256 before write: {:x}", result);
    println!("reading data from spi...");
    let mut data_from_spi = vec![0u8;total_size as usize];
    let mut bytes_read = 0;
    for i in 0..s {
        let buffer = w25q.read(s, per_write as u16).unwrap();
        bytes_read += buffer.len();
        for i in 0..buffer.len() {
            data_from_spi.push(buffer[i]);
        }
    }
    println!("bytes read: {}", bytes_read);
    println!("calculating sha256sum of data_from_spi");
    let mut hasher = Sha256::new();
    hasher.update(&data_from_spi.as_slice());
    let result = hasher.finalize();
    println!("sha256 after write: {:x}", result);
    for i in 0..data_to_write.len()/(per_write as usize) {
        println!("data_to_write");
        dump_vec_subset(&data_to_write, i as usize + per_write as usize);
        println!("data_from_spi");
        dump_vec_subset(&data_from_spi, i as usize + per_write as usize);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}