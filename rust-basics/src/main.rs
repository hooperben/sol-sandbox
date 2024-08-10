use num_bigint::BigUint;
use num_traits::ToPrimitive;


fn main() {
    println!("Hello, world!");
}

fn u64_array_to_biguint(arr: [u64; 4]) -> BigUint {
    let mut biguint = BigUint::from(0u64);
    for &part in arr.iter().rev() {
        biguint = (biguint << 64) + BigUint::from(part);
    }
    biguint
}

fn biguint_to_u64_array(biguint: BigUint) -> [u64; 4] {
    let mut arr = [0u64; 4];
    let bytes = biguint.to_bytes_le();
    for i in 0..bytes.len() {
        let shift = (i % 8) * 8;
        let idx = i / 8;
        arr[idx] += (bytes[i] as u64) << shift;
    }
    arr
}

fn modulo_u64_arrays(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let a_biguint = u64_array_to_biguint(a);
    let b_biguint = u64_array_to_biguint(b);
    let result_biguint = a_biguint % b_biguint;
    biguint_to_u64_array(result_biguint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_u64_arrays() {
        let a: [u64; 4] = [10, 0, 0, 0];
        let b: [u64; 4] = [3, 0, 0, 0];
        let result = modulo_u64_arrays(a, b);
        let expected: [u64; 4] = [1, 0, 0, 0];
        assert_eq!(result, expected);

        let a: [u64; 4] = [123456789, 0, 0, 0];
        let b: [u64; 4] = [100, 0, 0, 0];
        let result = modulo_u64_arrays(a, b);
        let expected: [u64; 4] = [89, 0, 0, 0];
        assert_eq!(result, expected);
    }
}