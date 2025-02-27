use proconio::{fastout, input};
use shanten_count::shanten::{Hand, JihaiHand, SuuhaiHand};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let suuhai_hash = SuuhaiHand::calc_shanten_to_all_partly_pattern();
    let jihai_hash = JihaiHand::calc_shanten_to_all_partly_pattern();

    for _ in 0..n {
        input! {
            t: [usize; 14],
        }

        let mut h = Hand::default();

        for t in t {
            h[t] += 1;
        }

        let s0 = h.shanten_standard(&suuhai_hash, &jihai_hash);
        let s1 = h.shanten_kokushimusou();
        let s2 = h.shanten_chiitoitsu();

        println!("{} {} {}", s0, s1, s2);
    }
}
