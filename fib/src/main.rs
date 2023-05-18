use std::collections::HashMap;
use rayon::prelude::*;

fn fib(n: u16) -> Result<u128, &'static str> {
    if n == 0 { return Err("n must be at least 1") }
    if n <= 2 { return Ok(1) }
    let mut hm: HashMap<u16, u128> = HashMap::from([(1, 1), (2, 1)]);
    return Ok(__fib(n, &mut hm));
}

fn __fib(n: u16, hm: &mut HashMap<u16, u128>) -> u128{
    if hm.contains_key(&n) { return hm[&n] }
    let result = __fib(n - 1, hm) + __fib(n - 2, hm);
    hm.insert(n, result);
    return hm[&n];
}

fn fib_threaded(n: u16) -> u128 {
    if n <= 2 { return 1 }
    let result = (1..=n).into_par_iter()
        .map(|i| {
            let mut hm = HashMap::new();
            hm.insert(1, 1);
            hm.insert(2, 1);
            for j in 3..=i {
                __fib(j, &mut hm);
            }
            hm[&i]
        })
        .collect::<Vec<u128>>()
        .pop()
        .unwrap();

    return result;
}


#[test]
fn test_fib_threaded() {
    use std::time::Instant;

    let test_start = Instant::now();
    let tests = [
        (80, 23416728348467685),
        (90, 2880067194370816120),
        (140, 81055900096023504197206408605),
        (141, 131151201344081895336534324866),
        (154, 68330027629092351019822533679447),
    ];
    for (n, expected) in &tests {
        let start = Instant::now();
        let result = fib_threaded(*n);
        let duration = start.elapsed();
        println!("fib_threaded({}) = {}, expected = {}, elapsed time: {:?}", n, result, expected, duration);
        assert_eq!(result, *expected);
    }
    let duration = test_start.elapsed();
    println!("All tests took {:?}\n", duration);
}


#[test]
fn test_fib() {
    use std::time::Instant;

    let test_start = Instant::now();
    let tests = [
        (80, 23416728348467685),
        (90, 2880067194370816120),
        (140, 81055900096023504197206408605),
        (141, 131151201344081895336534324866),
        (154, 68330027629092351019822533679447),
    ];
    for (n, expected) in &tests {
        let start = Instant::now();
        let result = fib(*n);
        let duration = start.elapsed();
        println!("fib({}) = {}, expected = {}, elapsed time: {:?}", n, result.unwrap(), expected, duration);
        assert_eq!(result.unwrap(), *expected);
    }
    let duration = test_start.elapsed();
    println!("All tests took {:?}\n", duration);
}

fn __grid_traveler(m: u16, n: u16, hm: &mut HashMap<(u16, u16), u128>) -> u128 {
    if hm.contains_key(&(m, n)) { return hm[&(m, n)] }
    if hm.contains_key(&(n, m)) { return hm[&(n, m)] }
    if m == 1 && n == 1 { return 1 }
    if m == 0 || n == 0 { return 0 }
    let result = __grid_traveler(m - 1, n, hm) + __grid_traveler(m, n - 1, hm);
    hm.insert((m, n), result);
    return hm[&(m, n)];
}

fn grid_traveler(m: u16, n: u16) -> u128 {
    if m == 1 && n == 1 { return 1 }
    if m == 0 || n == 0 { return 0 }
    let mut hm: HashMap<(u16, u16), u128> = HashMap::new();
    return __grid_traveler(m, n, &mut hm);
}

#[test]
fn test_grid_traveler() {
    use std::time::Instant;
    
    let test_start = Instant::now();
    let tests = [
        ((1, 1), 1),
        ((2, 3), 3),
        ((3, 2), 3),
        ((3, 3), 6),
        ((18, 18), 2333606220),
    ];
    for ((m, n), expected) in &tests {
        let start = Instant::now();
        let result = grid_traveler(*m, *n);
        let duration = start.elapsed();
        println!("grid_traveler({}, {}) = {}, expected = {}, elapsed time: {:?}", m, n, result, expected, duration);
        assert_eq!(result, *expected);
    }
    let duration = test_start.elapsed();
    println!("All tests took {:?}\n", duration);
}


fn main() {
    
}

