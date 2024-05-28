use std::result;

/// Converts a f64 to a byte array in a way that preserves the order of the f64s.
/// f64 is a 64-bit floating point number.
/// It is represented as a 64-bit number with 1 sign bit, 11 exponent bits, and 52 fraction bits.
/// We pack the exponent as 2 bytes and the fraction as 7 bytes.
/// If the number is negative, we flip all the bits.
fn f64_to_order_preserving_bytes(val: f64) -> [u8; 9] {
    let mut result = [0; 9];
    let val_bits = val.to_bits();
    let sign = (val_bits >> 63) as u8;
    let exponent = ((val_bits >> 52) & 0x7FF) as u16;
    let fraction = (val_bits & 0xFFFFFFFFFFFFF) as u64;
    result[0..2].copy_from_slice(&exponent.to_be_bytes());
    result[2..9].copy_from_slice(&fraction.to_be_bytes()[1..8]);
    if sign == 1 {
        // Flip all the bits
        result.iter_mut().for_each(|x| *x = !*x);
    }
    result
}

fn f64_to_order_preserving_bytes2(val: f64) -> [u8; 8] {
    let mut val_bits = val.to_bits();
    let sign = (val_bits >> 63) as u8;
    if sign == 1 {
        // Negative number so flip all the bits including the sign bit
        val_bits = !val_bits;
    } else {
        // Positive number. To distinguish between positive and negative numbers,
        // we flip the sign bit.
        val_bits ^= 1 << 63;
    }
    val_bits.to_be_bytes()
}

fn main() {
    {
        // Run small test
        let small = vec![-5.12, -3.14, 0.0, 3.14, 5.12];
        let mut small_with_bytes = small
            .iter()
            .map(|&x| (x, f64_to_order_preserving_bytes2(x)))
            .collect::<Vec<(f64, [u8; 8])>>();
        small_with_bytes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (f, bytes) in small_with_bytes {
            println!("{}: {:?}", f, bytes);
        }
    }

    let test_iteration = 100;
    for _ in 0..test_iteration {
        // Generate a random f64
        let num = 10000;
        let rand_vec = (0..num)
            .map(|_| {
                let f = rand::random::<f64>();
                let bytes = f64_to_order_preserving_bytes2(f);
                (f, bytes)
            })
            .collect::<Vec<(f64, [u8; 8])>>();

        let mut copy1 = rand_vec.clone();
        let mut copy2 = rand_vec.clone();

        // Sort copy1 based on the first column
        copy1.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Sort copy2 based on the second column
        copy2.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Check if the two copies are equal
        assert_eq!(copy1, copy2);
    }
}
