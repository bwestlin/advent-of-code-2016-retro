extern crate time;

use time::*;

pub fn measure<F>(f: F) where F: Fn() -> () {
    let start = precise_time_ns();
    let mut _times = 100;

    #[cfg(feature = "timeit")] {
        f();
        let dur_ns = precise_time_ns() - start;
        if dur_ns > 500_000_000 {
            _times /= 10;
        } else if dur_ns < 500_000 {
            _times *= 10;
        }
        for _ in 0..(_times - 1) {
            f();
        }
    }
    #[cfg(not(feature = "timeit"))] {
        f();
    }

    let dur_ns = precise_time_ns() - start;

    #[cfg(feature = "timeit")] {
        println!("It took: {}ms on average for {} times", (dur_ns / _times as u64) as f64 / 1_000_000.0, _times);
    }
    #[cfg(not(feature = "timeit"))] {
        println!("It took: {}ms", dur_ns as f64 / 1_000_000.0);
    }

}
