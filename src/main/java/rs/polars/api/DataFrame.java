package rs.polars.api;

public class DataFrame {

    private final long nativeObjectPointer;

//    private native long nativeNew();

    private native String print();

//    public DataFrame() {
//        this.nativeObjectPointer = nativeNew();
//    }

    public DataFrame(long nativeObjectPointer) {
        this.nativeObjectPointer = nativeObjectPointer;
    }

    @Override
    public String toString() {
        return print();
    }
}
