fn main() {
    // let mut count: u32 = 0;
    // let mut acc = 1;

    // while count < 2_u32.pow(30) {
    //     acc = acc + 2_u32.pow (30);
    //     acc = acc - 2_u32.pow (30);
    //     count = count + 1;
    // }

    let mut acc = 1;

    for i in 0..2_u32.pow(30) {
        for j in 0..2_u32.pow(30) {
            for k in 0..2_u32.pow(30) {
                for l in 0..2_u32.pow(30) {
                    for m in 0..2_u32.pow(30) {
                        for n in 0..2_u32.pow(30) {
                            for o in 0..2_u32.pow(30) {
                                for p in 0..2_u32.pow(30) {
                                    for q in 0..2_u32.pow(30) {
                                        for r in 0..2_u32.pow(30) {
                                            for s in 0..2_u32.pow(30) {
                                                for t in 0..2_u32.pow(30) {
                                                    for u in 0..2_u32.pow(30) {
                                                        for v in 0..2_u32.pow(30) {
                                                            acc = acc + 2_u32.pow(30);
                                                            acc = acc - 2_u32.pow(30);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}