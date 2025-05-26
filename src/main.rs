use rand::prelude::*;
use rand_chacha;
use rand_chacha::rand_core::RngCore;
use rand_chacha::ChaCha12Rng;
use std::io;

const MIN_LENGTH: i32 = 700;
const MIN_PRIME: i16 = 46; // ceil(sqrt(2048))

// RSA 2048 implementation :)
fn main() {
    let mut rng = ChaCha12Rng::from_os_rng();
    
    loop {
        let command = get_input("\n\nCommand [keygen, encode, decode]");

        if (command == "keygen") {
            keygen(&mut rng);
        }
        else if (command == "encode") {
            encode(&get_input("Message"), &get_input("Public Key"));
        }
        else if (command == "decode") {
            decode(&get_input("Message"), &get_input("Private Key"))
        }
        else {
            return;
        }
    }
}

fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn is_prime(n: i32) -> bool {
    if (n < 2) {
        return false
    }

    for x in 2..((n as f64).sqrt() as i32 + 1) {
        if (n % x == 0) {
            return false;
        }
    }
    true
}

fn random_i16(rng: &mut ChaCha12Rng) -> i16 {
    let mut number = rng.next_u32() as i64;
    number -= u32::MAX as i64;
    number as i16
}

fn random_prime(rng: &mut ChaCha12Rng) -> i32 {
    let mut number = random_i16(rng).abs();
    while (number < MIN_PRIME || !is_prime(number as i32)) {
        number = random_i16(rng).abs();
    }
    number as i32
}

fn coprime(first_number: i32, second_number: i32) -> i32 {
    let a;
    let b;
    if (first_number > second_number) {
        a = first_number;
        b = second_number;
    }
    else {
        a = second_number;
        b = first_number;
    }

    if (b == 0) {
        return a;
    }

    coprime(a % b, b)
}

fn is_coprime(a: i32, b: i32) -> bool {
    coprime(b.abs(), a.abs()) == 1
}

fn random_coprime(number: i32, rng: &mut ChaCha12Rng) -> i32 {
    let mut result = random_i16(rng).abs() as i32 % number;
    while (result > MIN_LENGTH || !is_coprime(result, number)) {
        result = random_i16(rng).abs() as i32 % number;
    }
    result
}

fn multiplicative_inverse(a: i32, m: i32) -> i32 {
    for x in 1..m {
        if (((a % m) as i64 * ((x % m) as i64)) % m as i64 == 1) {
            return x;
        }
    }
    0
}

fn keygen(rng: &mut ChaCha12Rng) {
    let p = random_prime(rng);
    let q = random_prime(rng);
    let n = p * q;
    let euler = (p - 1) * (q - 1);
    let d = random_coprime(euler, rng);
    let e = multiplicative_inverse(d, euler);

    println!("Public key:");
    println!("{},{}", n, e);
    println!("Private key:");
    println!("{},{}", d, n);
}

fn fast_pow_mod(base: i32, exponent: i32, modulator: i32) -> i32 {
    let mut answer: i64 = 1;
    for _i in 0..exponent {
        answer = (answer * base as i64) % modulator as i64;
    }
    (answer % modulator as i64) as i32
}

fn encode(msg: &String, public_key: &String) {
    let mut key_iterator = public_key.split(",");
    let n = key_iterator.next().unwrap().parse::<i32>().unwrap();
    let e = key_iterator.next().unwrap().parse::<i32>().unwrap();

    let mut first = true;
    for byte in msg.bytes() {
        let result = fast_pow_mod(byte as i32, e, n);

        if (!first) {
            print!(",");
        }
        first = false;
        print!("{}", result);
    }
}

fn decode(msg: &String, private_key: &String) {
    let mut key_iterator = private_key.split(",");
    let d = key_iterator.next().unwrap().parse::<i32>().unwrap();
    let n = key_iterator.next().unwrap().parse::<i32>().unwrap();

    for number in msg.split(",") {
            if (!number.is_empty()) {
            let result = fast_pow_mod(number.parse().unwrap(), d, n);
            print!("{}", result as u8 as char);
        }
    }
}