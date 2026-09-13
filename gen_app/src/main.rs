mod lfsr_ffi;

use lfsr_ffi::PERIOD;

fn main() {
    let bits = lfsr_ffi::generate();
    println!("First 10b: {:?}", &bits[0..10]);
    println!("Last 10b: {:?}", &bits[PERIOD-10..]);

    let _signal = lfsr_ffi::to_pml(&bits);
    //todo AKF KF GUI
}
