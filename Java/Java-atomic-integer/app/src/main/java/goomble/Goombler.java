package goomble;

import java.util.Random;
import java.util.concurrent.atomic.AtomicInteger;

public class Goombler {

    private AtomicInteger balance;
    private Random rand = new Random();
    private GoombleAccount goombleAccount;

    public Goombler(GoombleAccount goombleAccount, int initialBalance) {
        this.goombleAccount = goombleAccount;
        this.balance = new AtomicInteger(initialBalance);
    }

    public int getBalance() {
        return balance.get();
    }

    public void lucky() {
        int originalValue = balance.getAndUpdate(currentValue -> {
            if (currentValue > 0) {
                // Sleeping for a small, random amount of time here makes it more likely that
                // two or more threads will interleave here in interesting ways, thus creating
                // race conditions.
                try {
                    Thread.sleep(rand.nextInt(3));
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
                return currentValue - 1;
            }
            return 0;
        });
        // originalValue is the "old" value of the current user's balance before we
        // decremented it above. If this was >0 then we *did* decrement it by one
        // and we need to increment the Goomble account by one as well.
        if (originalValue > 0) {
            goombleAccount.increment();
        }
    }

}
