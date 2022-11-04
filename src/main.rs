use std::alloc::{self, Layout};
use std::thread::{self};
use std::time::{Duration, Instant};
use warp::{http, Filter};
use std::env::var;
use ctrlc;

#[tokio::main]
async fn main() {

    ctrlc::set_handler(move || {
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let work_time: u64 = var("WORK_TIME").unwrap().parse().unwrap();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let create_thread = warp::path!("add_work").map(move || {
        start_work_thread(work_time);
        warp::reply::with_status(
            format!("Added 1 unit of work for {} secs!", work_time),
            http::StatusCode::CREATED,
        )
    });
    let delete_thread = warp::path!("add_mem" / usize).map(move |value| {
        start_mem_thread(value, work_time);
        warp::reply::with_status(
            format!("Added {} bytes of memory for {} secs!", value, work_time),
            http::StatusCode::CREATED,
        )
    });
    warp::serve(create_thread.or(delete_thread))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

fn start_work_thread(work_time:u64) {
    thread::spawn(move || {
        println!("Starting CPU worker.");
        let mut _x = 0;
        let start_time = Instant::now();
        loop {
            if Instant::now() - Duration::from_secs(work_time) > start_time {
                println!("Terminating CPU worker.");
                break;
            }
            for _ in 0..1000000 {
                _x += 1;
                _x -= 1;
            }
        }
    });
}

fn start_mem_thread(mem_amount: usize, work_time:u64) {
    thread::spawn(move || {
        println!("Starting MEM worker with {} bytes of memory.", mem_amount);
        let pointer;
        let layout = Layout::array::<u8>(mem_amount).expect("Bad layout");

        // Try allocating to avoid making OOM abort
        pointer = unsafe { alloc::alloc(layout) };
        if pointer.is_null() {
            println!("OUT OF MEM ! Required mem: {}", mem_amount);
            return;
        }
        unsafe { alloc::dealloc(pointer, layout) };

        let value_to_fill: u8 = 0b10101010;
        let _my_vector = vec![value_to_fill; mem_amount];
        println!("Finished initializing for size {}", layout.size());

        thread::sleep(Duration::from_secs(work_time));

        println!(
            "Terminating mem thread with {} bytes of memory.",
            mem_amount
        );
    });
}
