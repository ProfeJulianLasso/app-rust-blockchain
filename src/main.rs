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
static COUNT_CORES_DEFAULT: u8 = 1;

fn main() {
  let args: Vec<String> = env::args().collect();

  // Obtiene la opción (create o check)
  let option = &args[1];

  // Verifica la opción y ejecuta la función correspondiente
  match option.as_str() {
    "create" => {
      // Inicia el cronómetro
      let start = Instant::now();

      // Obtiene los argumentos
      let info = &args[2];
      let level = &args[3];
      let stress = &args[4].parse::<usize>().unwrap();

      let nonce = Arc::new(Mutex::new(String::new()));
      let data = Arc::new(Mutex::new(String::new()));
      let hash = Arc::new(Mutex::new(String::new()));

      let cores = calculate_cores_to_use(&stress);

      // Crea los hilos
      let mut handles = vec![];
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
        let _ = handle.join().unwrap();
      }

      // Detiene el cronómetro
      let duration = start.elapsed();

      println!("Cantidad de CPU usadas: {} - {}%", cores, stress);
      println!("Nivel de dificultad: {}", level);
      println!("Nonce encontrado: {}", nonce.lock().unwrap());
      println!("Hash: {}", hash.lock().unwrap());
      println!("Datos resultantes: {}", data.lock().unwrap());
      println!("Tiempo transcurrido: {}s", duration.as_secs());
    },
    "check" => check(&args[2], &args[3], &args[4]),
    _ => error_option(),
  }
}

fn set_flag_to_true() {
  GLOBAL_FLAG.store(true, Ordering::SeqCst);
}

fn get_flag() -> bool {
  GLOBAL_FLAG.load(Ordering::SeqCst)
}

fn calculate_cores_to_use(stress: &usize) -> u8 {
  let num_threads = thread::available_parallelism().unwrap().get();
  let cores = (((num_threads * stress) / 100) as f32).ceil() as u8;
  if cores > num_threads as u8 {
    num_threads as u8
  } else {
    COUNT_CORES_DEFAULT
  }
}

fn create(
  info: &String,
  level: &String,
  nonce_final: &Arc<Mutex<String>>,
  data_final: &Arc<Mutex<String>>,
  hash_final: &Arc<Mutex<String>>,
) -> Result<(), ()> {
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
    return Ok(());
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
      return Ok(());
    }
  }

  Err(())
}

fn nonce_random() -> u32 {
  unsafe { randombytes_random() }
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
