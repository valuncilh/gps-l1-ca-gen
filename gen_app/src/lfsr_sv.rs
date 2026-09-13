//! Dynamic LFSR-generator C/A code GPS L1.
//!
//! Back link of selector phase G2 submit like parametr - one code -- all satilite
//!
//! Polynoms G1 and G2 fixed standart IS-GPS-200.

pub const PERIOD: usize = 1023;
const COUNT: usize = 10;
const G1_TAPS: [usize; 2] = [2, 9];
const G2_TAPS: [usize; 6] = [1, 2, 5, 7, 8, 9];
const INIT: u8 = 1;
// Генератор C/A кода для одного спутника GPS L1.
pub struct Lfsr {
    g1: [u8; COUNT],
    g2: [u8; COUNT],
    // Отводы селектора фазы G2 (динамический параметр спутника).
    phase_taps: (usize, usize),
}

impl Lfsr {
    // Создать генератор с заданными отводами селектора фазы G2.
    pub fn new(phase_taps: (usize, usize)) -> Self {
        Self {
            g1: [INIT; COUNT],
            g2: [INIT; COUNT],
            phase_taps,
        }
    }

    fn spin(bits: &mut [u8; COUNT]) {
        for i in (1..COUNT).rev() {
            bits[i] = bits[i - 1];
        }
        // bits[0] заполняется новым битом снаружи
    }

    // Один такт генератора — возвращает очередной бит C/A кода.
    fn tick(&mut self) -> u8 {
        let out_g1 = self.g1[9];

        let (t1, t2) = self.phase_taps;
        let g2i = self.g2[t1] ^ self.g2[t2];

        let bit = out_g1 ^ g2i;

        let mut new_g1 = 0;
        for &t in &G1_TAPS {
            new_g1 ^= self.g1[t];
        }

        let mut new_g2 = 0;
        for &t in &G2_TAPS {
            new_g2 ^= self.g2[t];
        }

        Self::spin(&mut self.g1);
        Self::spin(&mut self.g2);
        self.g1[0] = new_g1;
        self.g2[0] = new_g2;

        bit
    }

    // Сгенерировать один полный период (1023 бита).
    pub fn generate(&mut self) -> [u8; PERIOD] {
        let mut buf = [0u8; PERIOD];
        for i in 0..PERIOD {
            buf[i] = self.tick();
        }
        buf
    }
}
