fn main() {
    // let result = count_duplicates("aabbccff");
    println!("result: {:?}", totient(9999999985));
    // println!("result: {}", result);
    // println!("result: {}", result);
}
fn totient (n: u64) -> usize {
    let b_factors = factors(n);
    (1..=n)
        .filter(|x| !gcd_not_one(*x, &b_factors))
        .count()
}

fn gcd_not_one(a: u64, b_factors: &Vec<u64>) -> bool {
    let a_factors = factors(a);

    let mut a_i = 1;
    let mut b_i = 1;
    while a_i < a_factors.len() && b_i < b_factors.len() {
        let a_factor = a_factors[a_i];
        let b_factor = b_factors[b_i];
        if a_factor == b_factor {
            return true;
        }

        if a_factor < b_factor {
            a_i += 1;
        } else {
            b_i += 1;
        }
    }
    false
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut ret = vec![1];
    let mut divisor = 2;
    while n > 1 {
        if n % divisor == 0 {
            ret.push(divisor);
            n /= divisor;
        } else {
            divisor += 1
        }
    }
    ret
}