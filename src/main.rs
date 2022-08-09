use blake3::hash as blake3;
use libsodium_sys::randombytes_random;
use regex::Regex;
use std::env;
use std::time::Instant;
use thousands::Separable;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let mut nonce = nonce_random().to_string();
    let mut data = args[1].to_string().replace(
        "\"nonce\": null",
        &String::from("\"nonce\": ".to_owned() + &nonce),
    );
    let mut hash = blake3(&data.as_bytes());

    let pattern = format!(r"^(0){{{}}}", args[2]);
    let regex = Regex::new(pattern.as_str()).unwrap();
    let mut option = regex.is_match(&hash.to_string());

    let mut count = 0;
    while option == false {
        nonce = nonce_random().to_string();
        data = args[1].to_string().replace(
            "\"nonce\": null",
            &String::from("\"nonce\": ".to_owned() + &nonce),
        );
        hash = blake3(&data.as_bytes());
        option = regex.is_match(&hash.to_string());
        count += 1;
    }
    let duration = start.elapsed();

    println!("Nivel de dificultadad: {}", args[2]);
    println!("Nonce encontrado: {}", nonce);
    println!("Datos resultantes: {}", data);
    println!("Hash: {}", hash.to_string());
    println!("Cantidad de intentnos: {}", count.separate_with_dots());
    println!("Tiempo transcurrido: {:?}s", duration.as_secs());
}

fn nonce_random() -> u32 {
    unsafe {
        return randombytes_random();
    }
}
