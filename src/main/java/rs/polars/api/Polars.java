package rs.polars.api;

import java.util.List;
import java.util.Optional;

public class Polars {

    static {
        System.load("/home/zibin/IdeaProjects/java-polars/polars/target/debug/libpolars.so");
    }

    public static DataFrame readCsv(String filename, Optional<Boolean> hasHeader, Optional<List<String>> columns,
            Optional<List<String>> newColumns) {
        return new DataFrame(fromCSV(filename, hasHeader, columns, newColumns));
    }

    private static native long fromCSV(String filename, Optional<Boolean> hasHeader, Optional<List<String>> columns,
            Optional<List<String>> newColumns);

    public static void main(String[] args) {
        DataFrame df = readCsv("/home/zibin/IdeaProjects/java-polars/src/main/resources/data/data.csv",
                Optional.of(true), Optional.empty(), Optional.empty());
        System.out.println(df);
    }
}
