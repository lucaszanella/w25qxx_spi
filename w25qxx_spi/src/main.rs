mod w25q;
use w25q::W25Q;

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

    let mut s: [u8; 256] = [0;256];
    for i in 1..256 {
        s[i] = i as u8;
    }

    println!("gonna erase sector");
    let b = w25q.erase_sector(0, true);
    println!("erase sector: {}", b);
    println!("gonna read sector");
    let buffer = w25q.read(0, 256).unwrap();
    dump_vec(&buffer);

    let mut d:[u8;255] = [0;255];
    for i in 0..255 {
        d[i as usize] = i as u8;
    }
    let n = w25q.page_write(0, 0, &d);
    println!("did write: {} bytes", n);

    println!("gonna read changes");
    let buffer = w25q.read(0, 256).unwrap();
    dump_vec(&buffer);
    println!("buffer length: {}", buffer.len());
    println!("end");
}
