package day06
import java.io.File

fun solve(messages: List<String>): Pair<String, String> {
    val messageLength = 8;

    val initial = Array(messageLength) { IntArray(256) }
    val (part1, part2) = messages
        .fold(initial) { acc, message ->
            val bytes = message.toByteArray()
            for (i in 0 until messageLength) {
                acc[i][bytes[i].toInt()]++;
            }
            acc
        }
        .map {
            Pair(
                it.withIndex()
                    .maxBy { (_, c) -> c }
                    !!.index.toByte(),
                it.withIndex()
                    .filter { (_, c) -> c > 0 }
                    .minBy { (_, c) -> c }
                    !!.index.toByte()
            )
        }
        .unzip()

    return Pair(String(part1.toByteArray()), String(part2.toByteArray()))
}

fun main(args: Array<String>) {
    helpers.measure {
        val input = File(args[0]).readLines()

        val (part1, part2) = solve(input)

        println("Part1: ${part1}")
        println("Part2: ${part2}")
    }
}
