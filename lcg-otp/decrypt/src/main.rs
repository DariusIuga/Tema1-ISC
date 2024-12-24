use base64;
use std::str;
use std::time::Instant;

const MIN_PARAM: u32 = 1337;
const MAX_PARAM: u32 = 10000;
const FLAG_PREFIX: &str = "SpeishFlag{";

// LCG structure
struct LCG {
    a: u32,
    b: u32,
    state: u32,
    mod_val: u32,
}

impl LCG {
    fn new(a: u32, b: u32) -> Self {
        LCG {
            a,
            b,
            state: 0,
            mod_val: 1 << 16,
        }
    }

    fn next(&mut self) -> u16 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.b)) % self.mod_val;
        self.state as u16
    }
}

// XOR two byte slices
fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

fn main() {
    let b64_cipher = "R2t+aKTC9xO2dbO8pKGltKO83Zqx+fe0v/W7sLuw+/W2vfe5srmyubLfhLT3tr+wu6GivLr1tbS5vL7597e2u768+/W1tLm8vt+EpbK8pL2RubayrOGYvZWQr6Cmr+aAtaKYkJuS5eyFm+WWjqKl4OGPr6G4qA==";
    let cipher = base64::decode(b64_cipher).expect("Invalid base64 ciphertext");

    let start_time = Instant::now();

    for a in MIN_PARAM..=MAX_PARAM {
        for b in MIN_PARAM..=MAX_PARAM {
            let mut lcg = LCG::new(a, b);
            let key_length = cipher.len();
            let mut key = Vec::with_capacity(key_length);

            for _ in 0..(key_length / 2) {
                let state = lcg.next();
                key.extend(&state.to_le_bytes());
            }
            if key_length % 2 != 0 {
                let state = lcg.next();
                key.push(state as u8);
            }

            let plaintext = xor_bytes(&cipher, &key);

            if let Ok(text) = str::from_utf8(&plaintext) {
                if text.contains(FLAG_PREFIX) {
                    println!("Flag found with a={} b={}: {}", a, b, text);
                    return;
                }
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Flag not found. Time taken: {:?}", duration);
}
