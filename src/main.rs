use std::{
  env,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
  thread,
  time::Instant,
};

use blake3::hash as blake3;
use libsodium_sys::randombytes_random;
use regex::Regex;

static GLOBAL_FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
  let args: Vec<String> = env::args().collect();

  let option = &args[1];
  match option.as_str() {
    "create" => {
      let start = Instant::now();
      let info = &args[2].to_string();
      let level = &args[3].to_string();
      let stress = &args[4].to_string();

      let nonce = Arc::new(Mutex::new(String::new()));
      let data = Arc::new(Mutex::new(String::new()));
      let hash = Arc::new(Mutex::new(String::new()));

      let num_threads = thread::available_parallelism().unwrap().get();
      let mut cores: u8 =
        (((num_threads * stress.parse::<usize>().unwrap()) / 100) as f32).ceil() as u8;
      if cores == 0 {
        cores = 1;
      } else if cores > num_threads as u8 {
        cores = num_threads as u8;
      }

      // Crea los hilos
      // cargo add easy_parallel
      // Parallel::new()
      //   .each(0..cores, |_| create(&info, &level, &nonce, &data, &hash))
      //   .run();

      let mut handles = vec![];

      // Crea los hilos
      for _ in 0..cores {
        let info_clone = info.clone();
        let level_clone = level.clone();
        let nonce_clone = Arc::clone(&nonce);
        let data_clone = Arc::clone(&data);
        let hash_clone = Arc::clone(&hash);

        let handle = thread::spawn(move || {
          create(
            &info_clone,
            &level_clone,
            &nonce_clone,
            &data_clone,
            &hash_clone,
          )
        });

        handles.push(handle);
      }

      // Espera a que todos los hilos terminen
      for handle in handles {
        handle.join().unwrap();
      }

      let duration = start.elapsed();

      println!("Cantidad de CPU usadas: {} - {}%", cores, stress);
      println!("Nivel de dificultad: {}", level);
      println!("Nonce encontrado: {}", nonce.lock().unwrap());
      println!("Hash: {}", hash.lock().unwrap());
      println!("Datos resultantes: {}", data.lock().unwrap());
      println!("Tiempo transcurrido: {}s", duration.as_secs());
    },
    "check" => check(
      &args[2].to_string(),
      &args[3].to_string(),
      &args[4].to_string(),
    ),
    _ => error_option(),
  }
}

fn set_flag_to_true() {
  GLOBAL_FLAG.store(true, Ordering::SeqCst);
}

fn get_flag() -> bool {
  return GLOBAL_FLAG.load(Ordering::SeqCst);
}

fn create(
  info: &String,
  level: &String,
  nonce_final: &Arc<Mutex<String>>,
  data_final: &Arc<Mutex<String>>,
  hash_final: &Arc<Mutex<String>>,
) {
  let mut nonce = nonce_random().to_string();
  let mut data = info.replace(
    "\"nonce\":( null|null)",
    &String::from("\"nonce\": ".to_owned() + &nonce),
  );
  let mut hash = blake3(&data.as_bytes());

  let pattern = format!(r"^(0){{{}}}", level);
  let regex = Regex::new(pattern.as_str()).unwrap();
  let mut option = regex.is_match(&hash.to_string());

  if option == true {
    set_flag_to_true();
    *nonce_final.lock().unwrap() = nonce;
    *data_final.lock().unwrap() = data;
    *hash_final.lock().unwrap() = hash.to_string();
    return;
  }

  while option == false && get_flag() == false {
    nonce = nonce_random().to_string();
    data = info.replace(
      "\"nonce\": null",
      &String::from("\"nonce\": ".to_owned() + &nonce),
    );
    hash = blake3(&data.as_bytes());
    option = regex.is_match(&hash.to_string());

    if option == true {
      set_flag_to_true();
      *nonce_final.lock().unwrap() = nonce;
      *data_final.lock().unwrap() = data;
      *hash_final.lock().unwrap() = hash.to_string();
      return;
    }
  }
}

fn nonce_random() -> u32 {
  unsafe {
    return randombytes_random();
  }
}

fn check(
  info: &String,
  nonce: &String,
  hash: &String,
) {
  let data = info.replace(
    "\"nonce\": null",
    &String::from("\"nonce\": ".to_owned() + &nonce),
  );
  let hash_confirm = blake3(&data.as_bytes());
  if hash_confirm.to_string().eq(hash) {
    println!("OK");
  } else {
    println!("ERROR");
  }
}

fn error_option() {
  println!("Las opciones válidas son 'create' y 'check'");
}
