package goomble;

import java.util.Random;
import java.util.concurrent.atomic.AtomicInteger;

public class Goombler {

    // DON'T WANT THE REGION NUMBER; WANT A REGION COUNTER OBJECT!
    private int regionNumber;
    private int balance;
    private Random rand = new Random();

    public Goombler(int regionNumber, int initialBalance) {
        this.regionNumber = regionNumber;
        this.balance = initialBalance;
    }

    public int getBalance() {
        return balance;
    }

    public void lucky() {
        if (balance > 0) {
            // Sleeping for a small, random amount of time here makes it more likely that
            // two or more threads will interleave here in interesting ways, thus creating
            // race conditions.
            try {
                Thread.sleep(rand.nextInt(60));
            } catch (InterruptedException e) {
                // TODO Auto-generated catch block
                e.printStackTrace();
            }
            --balance;
        }
    }

}
