// const crypto = require (`crypto`)

// let nonce = 0
// let res = 1

// const difficulty = `0000010000000000000000000000000000000000000000000000000000000000`

// while (res > 0) {
//   const hash = crypto.createHash('sha256')
//     .update(nonce.toString())
//     .digest(Buffer)
//   // console.log('hash', hash.toString('hex'))
//   const compare = Buffer.from(difficulty, `hex`)
//   // console.log('compare', compare.toString(('hex')))
//   res = Buffer.compare(hash, compare)
//   // console.log(`res`, res)
//   nonce += 1
// }

// console.log(nonce-1)
use std::any::type_name;
use sha2::{Sha256, Digest};
use generic_array::{GenericArray, ArrayLength};
use hex;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut nonce = 0;
    let mut res = false;
    // let difficulty = String"0000010000000000000000000000000000000000000000000000000000000000";
    let difficulty = hex::decode("0000010000000000000000000000000000000000000000000000000000000000").expect("Problem");
    let difficulty = difficulty.as_slice();
    // let difficulty = [0x00, 0x00]
    // let difficulty = difficulty.as_slice();
    while !res {
        let mut hasher = Sha256::new();
        hasher.input(nonce.to_string().into_bytes());
        let result = hasher.result();
        // println!("difficulty = {}", difficulty);
        // println!("typeof = {}", type_of(&vec![result.as_slice()]));
        // println!("typeof result.as_slice() = {}", type_of(result.as_slice()));
        // println!("typeof difficulty = {}", type_of(difficulty));
        // println!("result.as_slice = {:?}", &result.as_slice()[..]);
        // println!("difficulty = {:?}", &difficulty[..]);
        res = result.as_slice() < difficulty;
        // res = vec![0x50, 0x60] < vec! [0x50, 0x40, 0x40];
        nonce = nonce + 1
    }

    println!("nonce = {}", nonce-1);

    // OLD
    // let mut hasher = Sha256::new();
    // let nonce = String::from("0");
    // hasher.input(nonce.into_bytes());
    // let result = hasher.result();
    // println!("sha = {:?}", result);
}