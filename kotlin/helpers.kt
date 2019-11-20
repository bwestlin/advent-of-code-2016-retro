package helpers
import kotlin.system.measureTimeMillis

fun measure(f: () -> Unit) {
    if (System.getenv("TIMEIT") != null) {
        var times = 100
        // Warmup
        for (i in 0..times) {
            val ms = measureTimeMillis {
                f()
            }
            if (ms >= 500) {
                times = 10
                break
            }
        }

        val start = System.nanoTime()
        for (i in 0..times) {
            f()
        }
        val end = System.nanoTime()

        println("It took: ${(end - start) / 1_000_000.0f / (times.toFloat())}ms on average for ${times} times")
    } else {
        val start = System.nanoTime()
        f()
        val end = System.nanoTime()
        println("It took: ${(end - start) / 1_000_000.0f}ms")
    }
}