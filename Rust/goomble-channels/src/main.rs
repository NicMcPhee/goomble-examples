use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use crossbeam_utils::thread;
use std::time::{Instant, Duration};
use std::hash::Hash;
use std::hash::Hasher;
use rand::Rng;
use rand::prelude::SliceRandom;
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

fn lucky(goombler: &mut Goombler, goomble_send: &Sender<u32>) {
    let mut rng = rand::thread_rng();
    // println!("Pressing a button for {}.", goombler.id);
    if goombler.balance > 0 {
        std::thread::sleep(Duration::from_millis(rng.gen::<u64>() % 3));
        goombler.balance -= 1;
        goomble_send.send(1).unwrap();
    }
}

fn main() {
    let (mut goomblers, initial_goomblers_total_balance) = init_goomblers(NUM_GOOMBLERS);
    println!("Initial total goomblers' balance was {}", initial_goomblers_total_balance);
    let (goomble_send, goomble_receive) = unbounded::<u32>();

    let start = Instant::now();

    // Building on https://docs.rs/crossbeam/0.8.1/crossbeam/thread/index.html
    let mut senders = Vec::new();
    thread::scope(|sc| {
        for goombler in &mut goomblers {
            let (s, r) = unbounded::<String>();
            senders.push(s);
            let gs = goomble_send.clone();
            sc.spawn(move |_| {
                for _ in r {
                    lucky(goombler, &gs);
                }
                drop(gs);
            });
        }
        drop(goomble_send);

        sc.spawn(move |_| {
            let mut goomble_balance = 0;
            for change in goomble_receive {
                goomble_balance += change;
                // println!("Updating Goomble balance to {}.", goomble_balance);
            }
            println!("Final Goomble balance was {}.", goomble_balance);
        });

        for _ in 0..NUM_PRESSES {
            let s = senders.choose(&mut rand::thread_rng()).unwrap();
            s.send(String::from("push")).unwrap();
        }

        for s in senders {
            drop(s);
        }
    }).unwrap();

    let duration = start.elapsed();

    let mut final_goombler_balance = 0;
    for goombler in &goomblers {
        println!("Goombler final balance = {}", goombler.balance);
        final_goombler_balance += goombler.balance;
    }
    println!("\tTotal final Goombler balance = {}", final_goombler_balance);

    println!("Total time was {:?}.", duration);
}
