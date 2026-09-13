//! Корреляционная обработка псевдослучайных последовательностей.
//!
//! Одна функция `correlate` считает и АКФ, и КФ — разница только в том,
//! какие два сигнала подаются на вход.

/// Циклическая корреляция двух сигналов ±1 одинаковой длины.
///
/// Возвращает массив значений для сдвигов `τ = 0..N-1`:
/// `result[τ] = Σ s1[i] * s2[(i + τ) mod N]`
pub fn correlate(s1: &[i8], s2: &[i8]) -> Vec<i32> {
    let n = s1.len();
    assert_eq!(n, s2.len(), "signals must have the same length");

    let mut result = vec![0i32; n];
    for tau in 0..n {
        let mut sum = 0i32;
        for i in 0..n {
            sum += (s1[i] as i32) * (s2[(i + tau) % n] as i32);
        }
        result[tau] = sum;
    }
    result
}

/// Печатает статистику по корреляции: пик, среднее, мин/макс боковых лепестков.
pub fn print_stats(name: &str, corr: &[i32]) {
    let peak = corr[0];
    let max = *corr.iter().max().unwrap();
    let min = *corr.iter().min().unwrap();
    let side_sum: i64 = corr[1..].iter().map(|&x| x as i64).sum();
    let side_avg = side_sum as f64 / (corr.len() - 1) as f64;

    println!("\n=== {} ===", name);
    println!("R(0)         = {}", peak);
    println!("max          = {}", max);
    println!("min          = {}", min);
    println!("side avg     = {:.4}", side_avg);
    println!("first 10:    {:?}", &corr[0..10]);
}
