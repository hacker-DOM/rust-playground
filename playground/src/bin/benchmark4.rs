// function findAllPrimeFactors(num) {
//     let _num = num
//     const primeFactors = []
//     for (let i = 2; i < Math.sqrt(num) + 1; i++) {
//       while (_num % i === 0) {
//         primeFactors.push(i)
//         _num = _num / i
//       }
//     }
//     // if (primeFactors.length === 0) {
//     //   return [num]
//     // } else {
//     //   return primeFactors
//     // }
//   } 
  
//   for (let i = 2; i < 2 ** 20; i++) {
//     // const res = findAllPrimeFactors(i).join('*')
//     // console.log (`${i}: ${res}`)
//     findAllPrimeFactors(i)
//   }
// use std::num::Float;

fn findAllPrimeFactors(num: u32) -> Vec<u32> {
    let mut _num = num;
    let mut primeFactors = vec![];
    let bound = (num as f32).sqrt().ceil() as u32;
    for i in 2..bound {
        while _num % i == 0 {
            primeFactors.push (i);
            _num = _num / i;
        }
    }

    if (primeFactors.len() == 0) {
        return vec![num];
    } else {
        return primeFactors;
    }
}

fn main() {
    for i in 2..2_u32.pow(20) {
        // let res = findAllPrimeFactors(i);
        // println!("{}: {:?}", i, res);
        findAllPrimeFactors(i);
    }
}