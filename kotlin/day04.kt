package day04
import java.io.File

data class Room(val name: String, val sectorId: Int, val checkSum: String)

fun validChecksum(r: Room) = r.name
    .filter { it >= 'a' }
    .groupingBy { it }
    .eachCount()
    .toList()
    .sortedWith(Comparator<Pair<Char, Int>> { (c1, n1), (c2, n2) ->
        when {
            n1 > n2 -> -1
            n1 < n2 -> 1
            else -> c1 - c2
        }
    })
    .take(5)
    .map { it.first }
    .joinToString("")
    .equals(r.checkSum)

fun decryptName(r: Room) = r.name
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

fun main(args: Array<String>) {
    helpers.measure {
        val input = File(args[0]).readLines().map(::parseRoom).toList()

        val part1 = input.filter(::validChecksum).sumBy { it.sectorId }

        val part2 = input.find { decryptName(it) == "northpole object storage" }?.sectorId

        println("Part1: ${part1}")
        println("Part2: ${part2}")
    }
}
