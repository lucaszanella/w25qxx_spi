mod w25q;
use w25q::W25Q;
use sha2::{Sha256, Digest};

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

fn main() {

    let mut w25q = W25Q::new(SPI_CHANNEL, SPEED).unwrap();
    let register_1 = w25q.read_status_register_1().unwrap();
    let register_2 = w25q.read_status_register_2().unwrap();
    let register_3 = w25q.read_status_register_3().unwrap();
    let manufacturer_id = w25q.read_manufacturer_id().unwrap();
    let jedec_id = w25q.read_jedec_id().unwrap();
    let unique_id = w25q.read_unique_id().unwrap();

    print!("register_1: ");
    dump_slice(&register_1);
    print!("register_2: ");
    dump_slice(&register_2);
    print!("register_3: ");
    dump_slice(&register_3);
    print!("manufacturer_id: ");
    dump_slice(&manufacturer_id);
    print!("jedec_id: ");
    dump_slice(&jedec_id);
    print!("unique_id:: ");
    dump_slice(&unique_id);
    
    //16mb or 128mbit
    let base2: i32 = 2;
    let total_size = base2.pow(24);
    let mut data = vec![0u8;total_size as usize];
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    println!("sha256 before write: {}", result.result_bytes());

}
