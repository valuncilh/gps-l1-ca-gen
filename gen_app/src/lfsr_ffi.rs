pub const PERIOD: usize = 1023;

unsafe extern "C" {
    fn LFSR(buf: *mut u8);
}

pub fn generate() -> [u8; PERIOD] {
    let mut buf = [0u8; PERIOD];
    unsafe { LFSR(buf.as_mut_ptr()); }
    buf
}

pub fn to_pml(bits: &[u8; PERIOD]) -> [i8; PERIOD] {
    let mut out = [0i8; PERIOD];
    for i in 0..PERIOD {
        out[i] = if bits[i] == 0 { 1 } else { -1 }; 
    }
    out
}
