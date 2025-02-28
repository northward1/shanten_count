use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use rustc_hash::FxHashMap;

#[derive(Default, PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone)]
pub struct Hand {
    m: SuuhaiHand,
    p: SuuhaiHand,
    s: SuuhaiHand,
    z: JihaiHand,
}

impl Hand {
    pub fn shanten_kokushimusou(&self) -> i8 {
        let mut yaochuu_tile_count = 0;
        let mut yaochuu_toitsu_count = 0;

        if self.m[0] >= 1 {
            yaochuu_tile_count += 1;
        }
        if self.m[0] >= 2 {
            yaochuu_toitsu_count += 1;
        }
        if self.p[0] >= 1 {
            yaochuu_tile_count += 1;
        }
        if self.p[0] >= 2 {
            yaochuu_toitsu_count += 1;
        }
        if self.s[0] >= 1 {
            yaochuu_tile_count += 1;
        }
        if self.s[0] >= 2 {
            yaochuu_toitsu_count += 1;
        }

        if self.m[8] >= 1 {
            yaochuu_tile_count += 1;
        }
        if self.m[8] >= 2 {
            yaochuu_toitsu_count += 1;
        }
        if self.p[8] >= 1 {
            yaochuu_tile_count += 1;
        }
        if self.p[8] >= 2 {
            yaochuu_toitsu_count += 1;
        }
        if self.s[8] >= 1 {
            yaochuu_tile_count += 1;
        }
        if self.s[8] >= 2 {
            yaochuu_toitsu_count += 1;
        }

        for i in 0..7 {
            if self.z[i] >= 1 {
                yaochuu_tile_count += 1;
            }

            if self.z[i] >= 2 {
                yaochuu_toitsu_count += 1;
            }
        }

        if yaochuu_toitsu_count >= 1 {
            return 12 - yaochuu_tile_count;
        } else {
            return 13 - yaochuu_tile_count;
        }
    }

    pub fn shanten_chiitoitsu(&self) -> i8 {
        let mut isolated_tile_count = 0;
        let mut toitsu_count = 0;

        for i in 0..9 {
            if self.m[i] >= 2 {
                toitsu_count += 1;
            } else if self.m[i] == 1 {
                isolated_tile_count += 1;
            }

            if self.p[i] >= 2 {
                toitsu_count += 1;
            } else if self.p[i] == 1 {
                isolated_tile_count += 1;
            }

            if self.s[i] >= 2 {
                toitsu_count += 1;
            } else if self.s[i] == 1 {
                isolated_tile_count += 1;
            }
        }

        for i in 0..7 {
            if self.z[i] >= 2 {
                toitsu_count += 1;
            } else if self.z[i] == 1 {
                isolated_tile_count += 1;
            }
        }

        if toitsu_count > 7 {
            toitsu_count = 7;
        }

        if toitsu_count + isolated_tile_count > 7 {
            isolated_tile_count = 7 - toitsu_count;
        }

        return 13 - 2 * toitsu_count - isolated_tile_count;
    }

    pub fn shanten_standard(
        &self,
        suuhai_pattern_dict: &FxHashMap<(SuuhaiHand, u8), u8>,
        jihai_pattern_dict: &FxHashMap<(JihaiHand, u8), u8>,
    ) -> i8 {
        let pattern = [0, 3, 6, 9, 12, 2, 5, 8, 11, 14];

        let mut ret = i8::MAX;
        let has_toitsu = |k| if k % 3 == 2 { 1 } else { 0 };

        for mc in pattern {
            for sc in pattern {
                for pc in pattern {
                    for jc in pattern {
                        let toitsu_count =
                            has_toitsu(mc) + has_toitsu(sc) + has_toitsu(pc) + has_toitsu(jc);

                        if toitsu_count == 1 && mc + sc + pc + jc == 14 {
                            let c = suuhai_pattern_dict
                                .get(&(self.m.clone(), mc as u8))
                                .unwrap()
                                + suuhai_pattern_dict
                                    .get(&(self.p.clone(), pc as u8))
                                    .unwrap()
                                + suuhai_pattern_dict
                                    .get(&(self.s.clone(), sc as u8))
                                    .unwrap()
                                + jihai_pattern_dict.get(&(self.z.clone(), jc as u8)).unwrap();

                            ret = std::cmp::min(ret, c as i8 - 1);
                        }
                    }
                }
            }
        }

        return ret;
    }
}

impl Index<usize> for Hand {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if index < 9 {
            &self.m[index]
        } else if index < 18 {
            &self.p[index - 9]
        } else if index < 27 {
            &self.s[index - 18]
        } else {
            &self.z[index - 27]
        }
    }
}

impl IndexMut<usize> for Hand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < 9 {
            &mut self.m[index]
        } else if index < 18 {
            &mut self.p[index - 9]
        } else if index < 27 {
            &mut self.s[index - 18]
        } else {
            &mut self.z[index - 27]
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let length = chars.len();

        let mut hand = Self::default();

        for i in 0..length {
            let n = chars[i];

            if ('0'..='9').contains(&n) {
                let mut value = n.to_digit(10).unwrap() as u8;

                if value == 0 {
                    value = 5;
                }

                value -= 1;

                let tile_type = (i + 1..length)
                    .map(|j| chars[j])
                    .filter(|&c| c == 'm' || c == 'p' || c == 's' || c == 'z')
                    .next();

                if tile_type.is_none() {
                    return Err(());
                }

                let tile_type = tile_type.unwrap();

                match tile_type {
                    'z' => {
                        if 1 <= value && value < 8 {
                            hand.z[value as usize] += 1;
                        } else {
                            return Err(());
                        }
                    }
                    c => {
                        if value <= 9 {
                            match c {
                                'm' => {
                                    hand.m[value as usize] += 1;
                                }
                                's' => {
                                    hand.s[value as usize] += 1;
                                }
                                'p' => {
                                    hand.p[value as usize] += 1;
                                }
                                _ => return Err(()),
                            }
                        } else {
                            return Err(());
                        }
                    }
                }
            }
        }

        return Ok(hand);
    }
}

#[derive(Default, PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone)]
pub struct SuuhaiHand([u8; 9]);
#[derive(Default, PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone)]
pub struct JihaiHand([u8; 7]);

impl SuuhaiHand {
    const LENGTH: usize = 9;

    pub fn check(&self) -> bool {
        (0..Self::LENGTH).filter(|&i| self[i] > 4).next().is_none()
    }

    pub fn all_partly_agari_pattern() -> Vec<Self> {
        let mut ret = vec![];

        let mut mentsu = vec![None];
        let mut toitsu = vec![None];

        for i in 0..Self::LENGTH - 2 {
            mentsu.push(Some([i, i + 1, i + 2]));
        }

        for i in 0..Self::LENGTH {
            mentsu.push(Some([i, i, i]));
            toitsu.push(Some([i, i]));
        }

        for m1 in mentsu.iter() {
            for m2 in mentsu.iter() {
                for m3 in mentsu.iter() {
                    for m4 in mentsu.iter() {
                        for t1 in toitsu.iter() {
                            let mut hand = Self::default();

                            if let Some(m1) = m1 {
                                for &t in m1 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(m2) = m2 {
                                for &t in m2 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(m3) = m3 {
                                for &t in m3 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(m4) = m4 {
                                for &t in m4 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(t1) = t1 {
                                for &t in t1 {
                                    hand[t] += 1;
                                }
                            }

                            if hand.check() {
                                ret.push(hand);
                            }
                        }
                    }
                }
            }
        }

        ret.sort();
        ret.dedup();

        return ret;
    }

    pub fn count(&self) -> u8 {
        self.0.iter().sum::<u8>()
    }

    pub fn calc_shanten_to_all_partly_pattern() -> FxHashMap<(Self, u8), u8> {
        let mut hash = FxHashMap::default();
        let mut q = std::collections::VecDeque::default();

        for partly_agari_pattern in Self::all_partly_agari_pattern() {
            let c = partly_agari_pattern.count();
            hash.insert((partly_agari_pattern.clone(), c), 0);
            q.push_front((partly_agari_pattern, c));
        }

        while let Some((pattern, c)) = q.pop_front() {
            let d = *hash.get(&(pattern.clone(), c)).unwrap();

            if pattern.count() < 14 {
                for i in 0..Self::LENGTH {
                    if pattern[i] == 4 {
                        continue;
                    }

                    let next_pattern = {
                        let mut next_pattern = pattern.clone();
                        next_pattern[i] += 1;
                        next_pattern
                    };

                    if let Some(&prev_dist) = hash.get(&(next_pattern.clone(), c)) {
                        if prev_dist > d {
                            hash.insert((next_pattern.clone(), c), d);
                        }
                    } else {
                        hash.insert((next_pattern.clone(), c), d);
                        q.push_front((next_pattern, c));
                    }
                }
            }

            if pattern.count() > 0 {
                for i in 0..Self::LENGTH {
                    if pattern[i] == 0 {
                        continue;
                    }

                    let next_pattern = {
                        let mut next_pattern = pattern.clone();
                        next_pattern[i] -= 1;
                        next_pattern
                    };

                    if !hash.contains_key(&(next_pattern.clone(), c)) {
                        hash.insert((next_pattern.clone(), c), d + 1);
                        q.push_back((next_pattern, c));
                    }
                }
            }
        }

        return hash;
    }
}

impl JihaiHand {
    const LENGTH: usize = 7;

    pub fn check(&self) -> bool {
        (0..Self::LENGTH).filter(|&i| self[i] > 4).next().is_none()
    }

    pub fn all_partly_agari_pattern() -> Vec<Self> {
        let mut ret = vec![];

        let mut mentsu = vec![None];
        let mut toitsu = vec![None];

        for i in 0..Self::LENGTH {
            mentsu.push(Some([i, i, i]));
            toitsu.push(Some([i, i]));
        }

        for m1 in mentsu.iter() {
            for m2 in mentsu.iter() {
                for m3 in mentsu.iter() {
                    for m4 in mentsu.iter() {
                        for t1 in toitsu.iter() {
                            let mut hand = Self::default();

                            if let Some(m1) = m1 {
                                for &t in m1 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(m2) = m2 {
                                for &t in m2 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(m3) = m3 {
                                for &t in m3 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(m4) = m4 {
                                for &t in m4 {
                                    hand[t] += 1;
                                }
                            }

                            if let Some(t1) = t1 {
                                for &t in t1 {
                                    hand[t] += 1;
                                }
                            }

                            if hand.check() {
                                ret.push(hand);
                            }
                        }
                    }
                }
            }
        }

        ret.sort();
        ret.dedup();

        return ret;
    }

    pub fn count(&self) -> u8 {
        self.0.iter().sum::<u8>()
    }

    pub fn calc_shanten_to_all_partly_pattern() -> FxHashMap<(Self, u8), u8> {
        let mut hash = FxHashMap::default();
        let mut q = std::collections::VecDeque::default();

        for partly_agari_pattern in Self::all_partly_agari_pattern() {
            let c = partly_agari_pattern.count();
            hash.insert((partly_agari_pattern.clone(), c), 0);
            q.push_front((partly_agari_pattern, c));
        }

        while let Some((pattern, c)) = q.pop_front() {
            let d = *hash.get(&(pattern.clone(), c)).unwrap();

            if pattern.count() < 14 {
                for i in 0..Self::LENGTH {
                    if pattern[i] == 4 {
                        continue;
                    }

                    let next_pattern = {
                        let mut next_pattern = pattern.clone();
                        next_pattern[i] += 1;
                        next_pattern
                    };

                    if let Some(&prev_dist) = hash.get(&(next_pattern.clone(), c)) {
                        if prev_dist > d {
                            hash.insert((next_pattern.clone(), c), d);
                        }
                    } else {
                        hash.insert((next_pattern.clone(), c), d);
                        q.push_front((next_pattern, c));
                    }
                }
            }

            if pattern.count() > 0 {
                for i in 0..Self::LENGTH {
                    if pattern[i] == 0 {
                        continue;
                    }

                    let next_pattern = {
                        let mut next_pattern = pattern.clone();
                        next_pattern[i] -= 1;
                        next_pattern
                    };

                    if !hash.contains_key(&(next_pattern.clone(), c)) {
                        hash.insert((next_pattern.clone(), c), d + 1);
                        q.push_back((next_pattern, c));
                    }
                }
            }
        }

        return hash;
    }
}

impl std::ops::Index<usize> for SuuhaiHand {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for SuuhaiHand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::fmt::Display for SuuhaiHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl From<&[u8; 9]> for SuuhaiHand {
    fn from(value: &[u8; 9]) -> Self {
        Self([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8],
        ])
    }
}

impl std::ops::Index<usize> for JihaiHand {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for JihaiHand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::fmt::Display for JihaiHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl From<&[u8; 7]> for JihaiHand {
    fn from(value: &[u8; 7]) -> Self {
        Self([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6],
        ])
    }
}
