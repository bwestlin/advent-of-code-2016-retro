import kotlin.system.measureTimeMillis

fun parseSides(l: String) = l.split(' ').filter(String::isNotEmpty).map(String::toInt).toTypedArray()

fun validTriangle(a: Int, b: Int, c: Int) = (a + b > c) && (a + c > b) && (b + c > a)

fun List<Array<Int>>.countValidTriangles() = this.filter { s -> validTriangle(s[0], s[1], s[2]) }.count()

fun List<Array<Int>>.transposed() = this.chunked(3).flatMap { c ->
    (0..2).map { i ->
        arrayOf(c[0][i], c[1][i], c[2][i])
    }
}

fun main() {
    val ms = measureTimeMillis {
        val input = generateSequence(::readLine).map(::parseSides).toList()

        val part1 = input.countValidTriangles()

        val part2 = input.transposed().countValidTriangles()

        println("Part1: ${part1}")
        println("Part2: ${part2}")
    }
    println("It took ${ms}ms")
}