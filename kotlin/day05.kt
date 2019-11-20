package day05
import java.io.File
import java.security.MessageDigest;

fun Byte.toUInt() = toInt() and 0xFF

fun solve(doorId: String): Pair<String, String> {
    val md5 = MessageDigest.getInstance("MD5")
    val prefix = intArrayOf(0xFF, 0xFF, 0XF0)
    val password1 = StringBuilder()
    val password2 = StringBuilder("________")

    outer@ for (index in 0..Int.MAX_VALUE) {
        md5.update("${doorId}${index}".toByteArray())
        val digest = md5.digest()
        for (i in prefix.indices) {
            if (prefix[i].and(digest[i].toUInt()) != 0) {
                continue@outer
            }
        }

        val nibble6 = digest[2].toInt().and(0xF)

        if (password1.length < 8) {
            password1.append("%01x".format(nibble6))
        }

        if (nibble6 < 8 && password2.get(nibble6) == '_') {
            val nibble7 = digest[3].toInt().shr(4).and(0xF)
            password2.setCharAt(nibble6, "%01x".format(nibble7).get(0))
            if (!password2.contains('_')) {
                break
            }
        }
    }

    return Pair(password1.toString(), password2.toString())
}

fun main(args: Array<String>) {
    helpers.measure {
        val input = File(args[0]).readLines().take(1)[0]

        val (part1, part2) = solve(input)

        println("Part1: ${part1}")
        println("Part2: ${part2}")
    }
}
