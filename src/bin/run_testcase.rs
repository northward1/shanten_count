use proconio::{fastout, input};
use shanten_count::shanten::Hand;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    for _ in 0..n {
        input! {
            t: [usize; 14],
        }

        let mut h = Hand::default();

        for t in t {
            h[t] += 1;
        }

        let s0 = h.shanten_standard();
        let s1 = h.shanten_kokushimusou();
        let s2 = h.shanten_chiitoitsu();

        println!("{} {} {}", s0, s1, s2);
    }
}
