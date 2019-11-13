import kotlin.system.measureTimeMillis

data class Room(val name: String, val sectorId: Int, val checkSum: String)

fun validChecksum(r: Room) = r.name
    .filter { it >= 'a' }
    .toList()
    .groupingBy { it }
    .eachCount()
    .toList()
    .groupBy { it.second }
    .mapValues { it.value.map { it.first } }
    .toList()
    .sortedByDescending { it.first }
    .flatMap { it.second.sorted() }
    .take(5)
    .joinToString("")
    .equals(r.checkSum)

fun decryptName(r: Room) = r.name
    .toList()
    .map { c ->
        when (c) {
            in 'a'..'z' -> 'a' + (r.sectorId + (c - 'a')) % ('z' - 'a' + 1)
            else -> ' '
        }
    }
    .joinToString("")

fun parseRoom(l: String): Room {
    val sectorIdSidx = l.lastIndexOf('-')
    val checksumSidx = l.lastIndexOf('[')
    return Room(
        l.substring(0, sectorIdSidx),
        l.substring(sectorIdSidx + 1, checksumSidx).toInt(),
        l.substring(checksumSidx + 1, checksumSidx + 6)
    )
}

fun main() {
    val ms = measureTimeMillis {
        val input = generateSequence(::readLine).map(::parseRoom).toList()

        val part1 = input.filter(::validChecksum).sumBy { it.sectorId }

        val part2 = input.find { decryptName(it) == "northpole object storage" }?.sectorId

        println("Part1: ${part1}")
        println("Part2: ${part2}")
    }
    println("It took ${ms}ms")
}
