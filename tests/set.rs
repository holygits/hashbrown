extern crate hashbrown;
extern crate rand;

use hashbrown::HashSet;
use rand::{Rng, SeedableRng, XorShiftRng};

#[test]
fn test_hashset_insert_remove() {
    let mut m: HashSet<Vec<char>> = HashSet::new();
    //let num: u32 = 4096;
    //let tx: Vec<Vec<u8>> = (0..num).map(|i| (i..(16 + i)).collect()).collect();
    let seed : [u8; 16] = [130, 220, 246, 217, 111, 124, 221, 189, 190, 234, 121, 93, 67, 95, 100, 43];

    let mut rng : XorShiftRng = SeedableRng::from_seed(seed);
    //let mut rng: XorShiftRng = XorShiftRng::new_unseeded();
    let tx: Vec<Vec<char>> = (0..4096)
             .map(|_|
                (rng.gen_ascii_chars().take(32).collect()))
             .collect();

    for _ in 0..32 {
        for i in 0..4096 {
            assert_eq!(m.contains(&tx[i].clone()), false);
            assert_eq!(m.insert(tx[i].clone()), true);
        }
        for i in 0..4096 {
            println!("removing {} {:?}", i, tx[i]);
            assert_eq!(m.remove(&tx[i]), true);
        }
    }
}
