use laches::LachesStore;
use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, Write},
    thread,
    time::{Duration, Instant},
};

fn tick(store_path: &str, update_interval: &Duration) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().write(true).open(&store_path).unwrap();

    let reader = BufReader::new(&file);
    let mut store: LachesStore = serde_json::from_reader(reader).unwrap();

    for process in &mut store.process_information {
        process.uptime += update_interval.as_millis() as u64;
    }

    let serialized_store = serde_json::to_string(&store).unwrap();
    file.write_all(serialized_store.as_bytes())?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: laches_mon <update_interval> <path/to/store>");
        return;
    }

    println!("info: running daemon (laches_mon)...");

    let update_interval: Duration = match args[1].parse() {
        Ok(num) => Duration::from_millis(num),
        Err(_) => {
            eprintln!("error: invalid argument(s) provided");
            eprintln!("usage: laches_mon <update_interval>");
            return;
        }
    };

    let file_path = args[2].as_str(); //todo: no validation of whether the path is actually in a valid form
    let mut last_tick = Instant::now();

    loop {
        let elapsed = last_tick.elapsed();
        if elapsed >= update_interval {
            tick(&file_path, &update_interval)
                .expect("error: error occured while monitoring windows");
            last_tick = Instant::now();
        }
        thread::sleep(Duration::from_millis(1));
    }
}
