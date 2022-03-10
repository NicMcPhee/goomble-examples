# Locking comparisons using Goomble as an example <!-- omit in toc -->

- [Overview](#overview)
- [Java version](#java-version)
  - [The basic approach](#the-basic-approach)
  - [Building and running](#building-and-running)
- [Rust version](#rust-version)
  - [The basic approach](#the-basic-approach-1)
  - [Building and running](#building-and-running-1)
- [Clojure](#clojure)
  - [The basic approach](#the-basic-approach-2)
  - [Building and running](#building-and-running-2)
## Overview

Saltzer and Kaashoek's _Principles of Computer Systems Design_ has a
problem set called "Goomble" that introduces a simple problem with
race conditions and locks.

The key procedure in the problem set is `LUCKY`,
which starts out as:

```text
  procedure LUCKY(account)
    if account.balance > 0 then
      account.balance <- account.balance-1
```

I also added a "global" Goomble account that is incremented whenever
a player account is decremented. The problem set goes on to propose
several possible locking solutions; we'll go with the account-based
locking since that is definitely the fastest.

This repository roughly implements that problem in several programming
languages and with different tools, to illustrate how varied the
approaches can be.

So far I have:

- Java using explicit locking
  - This is probably closest to the environment in the original
    problem set.
  - I actually screwed up and got a race condition on the
    Goomble account the first time.
  - The Java version is quite slow, and really scales terribly
    as we add players and run the simulation for longer.
- Rust using explicit locking
  - Rust's type system forced several important changes and made
    it essentially impossible to build the completely unlocked
    version full of race conditions.
  - The Rust version is _super_ fast compared to the others.
- Clojure using refs and transactions
  - This is probably closest to a database-style transaction.
  - The Clojure version isn't as fast as Rust, but is definitely
    a lot faster than Java.

I'd also like to add implementations using channels, but I just
haven't had time for that. In retrospect I should have also
set the number of players and maximum initial balances as
command line arguments so it would be easier to explore different
scales of the problem.

If you'd like to play with any of these fell free to clone this
repository. Below are instructions for compiling and running each
of the versions implemented so far. With all of them the first
build is likely to be noticeably slower than the others because
we'll need to download all the dependencies for that language.
Also, these assume you have the various languages installed. That
should be true in our lab, but if you want to run this on your own
computer you may need to install things like Rust and Leiningen.

## Java version

### The basic approach

I used `ReentrantLock`s to lock both the account-level balance
and the "global" Goomble balance (which is locked in `goombleAccount.increment()`):

```java
    private ReentrantLock lock = new ReentrantLock();

    public void lucky() {
        lock.lock();
        if (balance > 0) {
            --balance;
            goombleAccount.increment();
        }
        lock.unlock();
    }
```

### Building and running

To do anything with the Java version, make sure you first go into the
`Java` directory:

```
cd Java
```

To build and run the java version, one of the easier ways is to use

```text
./gradlew jar
```

to build a standalone JAR file which you can then run with or
without locks:

```text
java -jar app/build/libs/app.jar
java -jar app/build/libs/app.jar --lock
```

Both of these will report the total run time in milliseconds.

## Rust version

### The basic approach

I used Rust's `Mutex` (mutual exclusion) locks, again to lock both
the account-level balance and the "global" Goomble balance. A nice
feature here was that the locks automatically release themselves
when they go out of scope, reducing the likelihood of race conditions.
I also had to have asynchronous reference counters (`Arc`) on the
locks so they can be shared across multiple threads. Rust's type
system totally forced that on me, which was pretty cool.

```rust
fn lucky(goombler: &Goombler, goomble_balance: Arc<Mutex<u64>>) {
    let mut balance = goombler.balance.lock().unwrap();
    if *balance > 0 {
        *balance -= 1;
        // Frees up the lock on balance since we don't need that anymore.
        drop(balance);
        let mut goomble_balance = goomble_balance.lock().unwrap();
        *goomble_balance += 1;
    }
}
```

### Building and running

Go into the Rust directory and run `cargo run`. This will compile and run
the code, printing out some timing data at the end.

## Clojure

### The basic approach

I went with a fairly database-like transactional style here, using
`ref`s and `dosync` to create the "transaction".

```clojure
(defn lucky
  [goomble-balance goombler]
  (dosync
   (let [balance (:balance @goombler)]
     (when (pos? balance)
       (alter goombler assoc :balance (dec balance))
       (alter goomble-balance inc)))))
```

### Building and running

To build and run this, go into the Clojure directory and run `lein run`.