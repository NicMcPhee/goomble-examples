use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use rand::Rng;
use rand::prelude::SliceRandom;
use threadpool::ThreadPool;
use uuid::Uuid;

struct Goombler {
    id: Uuid,
    balance: u32 // Mutex<u32>
}
impl PartialEq for Goombler {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Goombler {}

impl Hash for Goombler {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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
        let goombler = Goombler { 
            id: Uuid::new_v4(),
            balance: initial_random_balance 
        }; //  Mutex::new(initial_random_balance) };
        initial_goomblers_total_balance += initial_random_balance;
        goomblers.push(goombler);
    }

    return (goomblers, initial_goomblers_total_balance);
}

fn lucky(goombler: &Goombler) {
    println!("Pressing a button for {}.", goombler.id);
    let mut balance = goombler.balance;
    if balance > 0 {
        balance -= 1;
        // // Frees up the lock on balance since we don't need that anymore.
        // drop(balance);
        // let mut goomble_balance = goomble_balance.lock().unwrap();
        // *goomble_balance += 1;
    }
}

fn main() {
    let (goomblers, initial_goomblers_total_balance) = init_goomblers(NUM_GOOMBLERS);

    let mut channel_map = HashMap::new();
    for goombler in goomblers {
        let (tx, rx) = mpsc::channel();
        channel_map.insert(goombler, (tx, rx));
    }

    let thread_pool = ThreadPool::new(NUM_THREADS);

    for goombler in goomblers {
        let (tx, rx) = channel_map.get(&goombler).unwrap();
        thread_pool.execute(move|| {
            for received in rx {
                lucky(&goombler);
            }
        })
    }

    for _ in 0..NUM_PRESSES {
        // let goomblers = Arc::clone(&arc_goomblers);
        // let goomble_balance = Arc::clone(&goomble_balance);
        let goombler = goomblers.choose(&mut rand::thread_rng()).unwrap();
        let (tx, rx) = channel_map.get(goombler).unwrap();
        tx.send(String::from("push")).unwrap();
    }

    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }
}
