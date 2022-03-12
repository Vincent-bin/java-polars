package rs.polars.api;

public class Polars {

    static {
        System.load("/home/zibin/IdeaProjects/java-polars/polars/target/debug/libpolars.so");
    }

    public static DataFrame readCsv(String filename) {
        return new DataFrame(fromCSV(filename));
    }

    private static native long fromCSV(String filename);

    public static void main(String[] args) {
        DataFrame df = readCsv("/home/zibin/IdeaProjects/java-polars/src/main/resources/data/data.csv");
        System.out.println(df);
    }
}
