import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class GiftShop {

    public static void main(String[] args) throws IOException {
        Main main = new Main();
        main.partOne();
    }

    private void partOne() throws IOException {
        long sum = 0;
        String input = Files.readString(Path.of("day_02/inputs.txt")).trim();

        for (String range : input.split(",")) {
            String[] parts = range.split("-");
            long start = Long.parseLong(parts[0]);
            long end = Long.parseLong(parts[1]);

            for (long n = start; n <= end; n++) {
                String s = Long.toString(n);
                if (s.length() % 2 != 0) continue;  // must be even length

                int half = s.length() / 2;
                String left = s.substring(0, half);
                String right = s.substring(half);

                if (left.equals(right)) {
                    sum += n;
                }
            }
        }

        System.out.println(sum);
    }

    private void partTwo() {
        // Implementation for part two
    }
}
