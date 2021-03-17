mod w25q;
use w25q::W25Q;

const SPI_CHANNEL: i32 = 0;
//2MHz
const SPEED: i32 = 2000000;

fn dump_slice(slice: &[u8]) {
    for i in 0..slice.len() {
        if i!=slice.len() {
            print!("{},", slice[i]);
        } else { 
            print!("{}", slice[i]);
        }
    }
    println!("");
}

fn main() {

    let w25q = W25Q::new(SPI_CHANNEL, SPEED).unwrap();
    let manufacturer_id = w25q.read_manufacturer_id();
    let jedec_id = w25q.read_jedec_id();
    print!("manufacturer_id: ");
    dump_slice(&manufacturer_id);
    print!("jedec_id: ");
    dump_slice(&jedec_id);
    println!("end");
}
