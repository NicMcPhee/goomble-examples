package goomble;

import java.util.Random;

public class Goombler {

    private int balance;
    private Random rand = new Random();
    private GoombleAccount goombleAccount;

    public Goombler(GoombleAccount goombleAccount, int initialBalance) {
        this.goombleAccount = goombleAccount;
        this.balance = initialBalance;
    }

    public int getBalance() {
        return balance;
    }

    public void lucky() {
        // We're not going to lock here, so bad things will happen.
        // if (GoombleSimulation.useLocks) {
        //     lock.lock();
        // }
        if (balance > 0) {
            // Sleeping for a small, random amount of time here makes it more likely that
            // two or more threads will interleave here in interesting ways, thus creating
            // race conditions.
            try {
                Thread.sleep(rand.nextInt(3));
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            --balance;
            goombleAccount.increment();
        }
        // if (GoombleSimulation.useLocks) {
        //     lock.unlock();
        // }
    }

}
