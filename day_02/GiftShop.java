import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashSet;
import java.util.Set;

public class GiftShop {

    public static void main(String[] args) throws IOException {
        GiftShop giftShop = new GiftShop();
        giftShop.partOne();
        giftShop.partTwo();
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

        System.out.println("Solution part one: " + sum);
    }

    private void partTwo() throws IOException {
        long sum = 0;
        Set<Long> invalids = new HashSet<>();
        String input = Files.readString(Path.of("day_02/inputs.txt")).trim();

        for (String range : input.split(",")) {
            String[] parts = range.split("-");
            long start = Long.parseLong(parts[0]);
            long end = Long.parseLong(parts[1]);

            for (long n = start; n <= end; n++) {
                String s = Long.toString(n);
                int len = s.length();

                for (int i = 1; i <= len / 2; i++) {
                    if (len % i != 0) continue;

                    String seq = s.substring(0, i);

                    if (seq.repeat(len / i).equals(s)) {
                        invalids.add(n);
                        break;
                    }
                }
            }
        }
        for (Long i : invalids) {
            sum += i;
        }
        System.out.println("Solution part two: " + sum);
    }
}
