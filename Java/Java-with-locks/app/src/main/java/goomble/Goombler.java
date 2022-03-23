package goomble;

import java.util.Random;
import java.util.concurrent.locks.ReentrantLock;

public class Goombler {

    private int balance;
    private Random rand = new Random();
    private GoombleAccount goombleAccount;
    private ReentrantLock lock = new ReentrantLock();

    public Goombler(GoombleAccount goombleAccount, int initialBalance) {
        this.goombleAccount = goombleAccount;
        this.balance = initialBalance;
    }

    public int getBalance() {
        return balance;
    }

    public void lucky() {
        lock.lock();
        if (balance > 0) {
            // Sleeping for a small, random amount of time here makes it more likely that
            // two or more threads will interleave here in interesting ways, thus creating
            // race conditions.
            try {
                Thread.sleep(rand.nextInt(60));
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            --balance;
            goombleAccount.increment();
        }
        lock.unlock();
    }

}
