import com.github.ajalt.mordant.rendering.TextColors.*;
import com.github.ajalt.mordant.terminal.*;
import kotlin.time.measureTimedValue


fun <T, R> run(label: String, getInput: () -> T, challenge: (T) -> R, expectedValue: R? = null) {
    val t = Terminal()
    val (input, inputTime) = measureTimedValue { getInput() };
    val (result, challengeTime) = measureTimedValue { challenge(input) }

    val success = if (expectedValue == null) true else result == expectedValue;

    val labelStyle = if (success) (black on brightGreen) else (black on red);

    t.println(labelStyle("$label: ${if (success) "Ok" else "Bad"}"))
    t.println("\tValue: $result")
    if (expectedValue != null) t.println("\tExpectedValue: $expectedValue");
    t.println("\tGetInputTook: $inputTime")
    t.println("\tChallengeTook: $challengeTime")

}

fun main() {
    run("Day1, Part1", ::day01_getInput, ::day01_part1, 55386);
    run("Day1, Part2", ::day01_getInput, ::day01_part2, 54824);
}