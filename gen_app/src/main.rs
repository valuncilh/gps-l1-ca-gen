mod corr;
mod lfsr_ffi;
mod lfsr_sv;

use lfsr_sv::Lfsr;
use std::time::Instant;

fn main() {
    // --- 1. Свой спутник SV2 (отводы 3 @ 7 → индексы 2 и 6) ---
    let mut sv2 = Lfsr::new((2, 6));
    let bits_sv2 = sv2.generate();

    let bits_c = lfsr_ffi::generate();
    let equal = bits_sv2.iter().zip(bits_c.iter()).all(|(a, b)| a == b);
    println!("Rust == C: {}", equal);
    println!("SV2 first 10: {:?}", &bits_sv2[0..10]);

    // --- 2. Второй спутник SV65 ---
    // TODO: вписать реальные отводы из таблицы 1.3 для SV ID 65.
    // Пример (индексы, не реальные значения):
    let mut sv65 = Lfsr::new((3, 8));
    let bits_sv65 = sv65.generate();
    println!("SV65 first 10: {:?}", &bits_sv65[0..10]);

    // --- 3. Перевод 0/1 → ±1 ---
    let s_sv2  = lfsr_ffi::to_pm1(&bits_sv2);
    let s_sv65 = lfsr_ffi::to_pm1(&bits_sv65);

    // --- 4. АКФ (сигнал сам с собой) ---
    let start = Instant::now();
    let akf = corr::correlate(&s_sv2, &s_sv2);
    let t_akf = start.elapsed();
    corr::print_stats("AKF SV2", &akf);
    println!("time: {:?}", t_akf);

    // --- 5. КФ (SV2 vs SV65) ---
    let start = Instant::now();
    let kf = corr::correlate(&s_sv2, &s_sv65);
    let t_kf = start.elapsed();
    corr::print_stats("KF SV2 vs SV65", &kf);
    println!("time: {:?}", t_kf);

    // --- 6. Сохранение в файлы для построения графиков ---
    save_csv("akf.csv", &akf);
    save_csv("kf.csv", &kf);
    println!("\nSaved: akf.csv, kf.csv");
}

fn save_csv(path: &str, data: &[i32]) {
    use std::fs::File;
    use std::io::{BufWriter, Write};

    let file = File::create(path).expect("cannot create file");
    let mut w = BufWriter::new(file);
    for (tau, &v) in data.iter().enumerate() {
        writeln!(w, "{},{}", tau, v).expect("write failed");
    }
}
