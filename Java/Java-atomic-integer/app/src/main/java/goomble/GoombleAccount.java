package goomble;

import java.util.concurrent.atomic.AtomicInteger;

public class GoombleAccount {

    private AtomicInteger atomicBalance = new AtomicInteger(0);

    public void increment() {
        atomicBalance.incrementAndGet();
    }

    public int getAtomicBalance() {
        return atomicBalance.get();
    }

}
