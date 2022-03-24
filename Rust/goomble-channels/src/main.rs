/*
 * I SHOULD TOTALLY REPLACE ALL THIS WITH THE CROSSBEAM CRATE
 * AS IT LOOKS LIKE IT WILL BE MUCH EASIER TO WORK WITH IN THE
 * WAY THAT I'M THINKING OF THINGS.
 * 
 * https://docs.rs/crossbeam-channel/0.5.4/crossbeam_channel/
 */

use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
// use std::sync::mpsc;
// use std::sync::mpsc::Sender;
// use std::sync::mpsc::Receiver;
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;
use std::hash::Hash;
use std::hash::Hasher;
use rand::Rng;
use rand::prelude::SliceRandom;
use threadpool::ThreadPool;
use uuid::Uuid;

struct Goombler {
    id: Uuid,
    balance: u32, // Mutex<u32>
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
            balance: initial_random_balance,
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

    // let mut channel_map = HashMap::new();
    // for goombler in goomblers.iter() {
    //     let (s, r) = unbounded::<String>();
    //     channel_map.insert(goombler, (s, r));
    // }

    // Building on https://docs.rs/crossbeam/0.8.1/crossbeam/thread/index.html
    let mut senders = Vec::new();
    thread::scope(|sc| {
        for goombler in &goomblers {
            let (s, r) = unbounded::<String>();
            senders.push(s);
            sc.spawn(move |_| {
                for _ in r {
                    lucky(&goombler);
                }
            });
        }
    }).unwrap();

    for _ in 0..NUM_PRESSES {
        // let goomblers = Arc::clone(&arc_goomblers);
        // let goomble_balance = Arc::clone(&goomble_balance);
        // let goombler = goomblers.choose(&mut rand::thread_rng()).unwrap();
        // let (s, r) = channel_map.get(goombler).unwrap();
        let s = senders.choose(&mut rand::thread_rng()).unwrap();
        s.send(String::from("push")).unwrap();
        // let s = s.clone();
        // thread_pool.execute(move|| {
        //     tx.send(String::from("push")).unwrap();
        //     // lucky(goombler);
        //     // lucky(goombler, goomble_balance);
        // });
    }

    for goombler in &goomblers {
        println!("Goombler final balance = {}", goombler.balance);
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
