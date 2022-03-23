package goomble;

import java.util.concurrent.atomic.AtomicInteger;

public class GoombleAccount {

    private int balance = 0;
    private AtomicInteger atomicBalance = new AtomicInteger(0);

    public void increment() {
        ++balance;
        atomicBalance.incrementAndGet();
    }

    public int getBalance() {
        return balance;
    }

    public int getAtomicBalance() {
        return atomicBalance.get();
    }

}
