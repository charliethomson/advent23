import java.io.File
import java.lang.Exception
import java.lang.StringBuilder

fun day01_getInput(): List<String> = File("../rust/input/2023/day1.txt").readLines()
//fun day01_getInput(): List<String> = buildList {
//    add("two1nine")
//    add("eightwothree")
//    add("abcone2threexyz")
//    add("xtwone3four")
//    add("4nineeightseven2")
//    add("zoneight234")
//    add("7pqrstsixteen")
//    add("2oneigh?three4")
//}


fun day01_part1(lines: List<String>): Int {
    return lines.map { line ->
        var first = -1
        var last = -1

        line.forEach inner@{
            if (!it.isDigit()) return@inner

            if (first == -1) first = it.digitToInt()
            last = it.digitToInt()
        }

        return@map first * 10 + last;
    }.sum()
}

val numbers: List<String> = buildList {
    add("one")
    add("two")
    add("three")
    add("four")
    add("five")
    add("six")
    add("seven")
    add("eight")
    add("nine")
}

class Scanner {
    var buffer: StringBuilder = StringBuilder();
    var candidates: List<String> = ArrayList(numbers);

    val remove: Boolean
        get() = candidates.isEmpty();

    constructor(buffer: Char) {
        this.buffer.append(buffer).toString();
        this.candidates = this.candidates.filter { it.startsWith(buffer) }.map { it.slice(1..<it.length) }
    }

    private fun toInt(): Int {
        return when (this.buffer.toString()) {
            "one" -> 1
            "two" -> 2
            "three" -> 3
            "four" -> 4
            "five" -> 5
            "six" -> 6
            "seven" -> 7
            "eight" -> 8
            "nine" -> 9
            else -> throw Exception("Invalid buffer")
        }
    }

    fun scan(char: Char): Int? {
        buffer.append(char);
        this.candidates = this.candidates.filter { it.startsWith(char) }.map { it.slice(1..<it.length) }
        val found = this.candidates.any { it.isEmpty() };
        this.candidates = this.candidates.filter { it.isNotEmpty() };
        return if (found) toInt() else null;
    }
}

fun day01_part2(lines: List<String>): Int {
    var sum = 0;
    for (line in lines) {
        var first = -1
        var last = -1

        val scanners = ArrayList<Scanner>(5);
        for (ch in line) {
            if (ch.isDigit()) {
                if (first == -1) first = ch.digitToInt()
                last = ch.digitToInt()
                continue;
            }

            for (scanner in scanners) {
                val result = scanner.scan(ch) ?: continue;
                if (first == -1) first = result;
                last = result;
            }

            scanners.add(Scanner(ch))
            scanners.removeAll { it.remove };
        }
        sum += first * 10 + last;
    }

    return sum;
}