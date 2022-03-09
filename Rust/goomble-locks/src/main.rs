use std::sync::{Arc, Mutex};
use rand::Rng;
use rand::seq::SliceRandom;
use threadpool::ThreadPool;

struct Goombler {
    balance: Mutex<u32>
}

static NUM_GOOMBLERS: u32 = 12;
static MAX_BALANCE: u32 = 100;
static NUM_PRESSES: u32 = NUM_GOOMBLERS * MAX_BALANCE;
static NUM_THREADS: usize = 10;

fn init_goomblers(num_goomblers: u32) -> (Vec<Goombler>, u32) {
    let mut rng = rand::thread_rng();
    let mut initial_goomblers_total_balance = 0;

    let mut goomblers: Vec<Goombler> = Vec::new();
    for _ in 0..num_goomblers {
        let initial_random_balance = rng.gen::<u32>() % MAX_BALANCE;
        let goombler = Goombler { balance: Mutex::new(initial_random_balance) };
        initial_goomblers_total_balance += initial_random_balance;
        goomblers.push(goombler);
    }

    return (goomblers, initial_goomblers_total_balance);
}

fn lucky(goombler: &Goombler) {
    let mut balance = goombler.balance.lock().unwrap();
    // println!("Doing lucky() for goombler with balance {}.", balance);
    if *balance > 0 {
        *balance -= 1;
    }
}

fn main() {
    println!("Hello, world!");

    let (goomblers, initial_goomblers_total_balance) = init_goomblers(NUM_GOOMBLERS);

    let arc_goomblers = Arc::new(goomblers);

    let thread_pool = ThreadPool::new(NUM_THREADS);
    for _ in 0..NUM_PRESSES {
        let goomblers = Arc::clone(&arc_goomblers);
        thread_pool.execute(move|| {
            let goombler = goomblers.choose(&mut rand::thread_rng()).unwrap();
            lucky(goombler);
        });
    }
    thread_pool.join();

    println!("Initial goomblers total balance is {}.", initial_goomblers_total_balance);
    let mut totalBalance = 0;
    // WHY THE &* HERE?!?
    for goombler in &*(Arc::clone(&arc_goomblers)) {
        println!("Howdy");
    }
}
