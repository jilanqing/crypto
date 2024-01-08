use crate::crypto::cfg::sha_cfg::*;

pub enum ShaType {
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}

pub fn sha(message: &str, sha_type: ShaType) -> String {
    let mut message_padding: Vec<u8> = Vec::from(message);

    padding(&mut message_padding, &sha_type);
    let mut hash_str = String::new();
    match sha_type {
        ShaType::SHA224 => {
            let hash: [u32; 8] = iteration_64(&message_padding, &HASH224_INIT_VALUES, &K_224_256);
            for word in hash.into_iter().take(7) {
                hash_str.push_str(&format!("{:08x}", word))
            }
        }
        ShaType::SHA256 => {
            let hash: [u32; 8] = iteration_64(&message_padding, &HASH256_INIT_VALUES, &K_224_256);
            for word in hash {
                hash_str.push_str(&format!("{:08x}", word))
            }
        }
        ShaType::SHA384 => {
            let hash: [u64; 8] = iteration_80(&message_padding, &HASH384_INIT_VALUES, &K_384_512);
            for word in hash.into_iter().take(6) {
                hash_str.push_str(&format!("{:016x}", word))
            }
        }
        ShaType::SHA512 => {
            let hash: [u64; 8] = iteration_80(&message_padding, &HASH512_INIT_VALUES, &K_384_512);
            for word in hash {
                hash_str.push_str(&format!("{:016x}", word))
            }
        }
    };

    hash_str
}

fn padding(message: &mut Vec<u8>, sha_type: &ShaType) {
    let alignment: u32;
    let remainer_expect: u32;
    let message_bits = (message.len() as u32) * u8::BITS;
    let mut message_length: Vec<u8>;
    match sha_type {
        ShaType::SHA224 | ShaType::SHA256 => {
            alignment = 512;
            remainer_expect = 448;
            message_length = Vec::from((message_bits as u64).to_be_bytes());
        }
        ShaType::SHA384 | ShaType::SHA512 => {
            alignment = 1024;
            remainer_expect = 896;
            message_length = Vec::from((message_bits as u128).to_be_bytes());
        }
    };
    let remainer = message_bits % alignment;
    let bits_padding = if remainer == remainer_expect {
        alignment
    } else {
        if remainer > remainer_expect {
            alignment + remainer_expect - remainer
        } else {
            remainer_expect - remainer
        }
    };
    message.push(0x80);
    for _ in 1..(bits_padding as u32) / u8::BITS {
        message.push(0x00);
    }
    message.append(&mut message_length);
}

fn iteration_64(message_padding: &Vec<u8>, hash_init_values: &[u32; 8], k: &[u32; 64]) -> [u32; 8] {
    let mut w: [u32; 64] = [0; 64];
    let mut m: Vec<u32> = Vec::new();
    let mut hash = hash_init_values.clone();

    for chunk in message_padding.chunks(32 / 8) {
        m.push(u32::from_be_bytes(chunk.try_into().unwrap()));
    }

    // main loop
    for mi in 0..((m.len() * 32) / 512) {
        for i in 0..16 {
            w[i] = m[(mi * 16)..(mi * 16 + 16)][i];
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = s1 + w[i - 7] + s0 + w[i - 16];
        }

        let mut a = hash[0];
        let mut b = hash[1];
        let mut c = hash[2];
        let mut d = hash[3];
        let mut e = hash[4];
        let mut f = hash[5];
        let mut g = hash[6];
        let mut h = hash[7];

        for i in 0..64 {
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h + s1 + ch + k[i] + w[i];
            let t2 = s0 + maj;

            h = g;
            g = f;
            f = e;
            e = d + t1;
            d = c;
            c = b;
            b = a;
            a = t1 + t2;
        }

        hash[0] += a;
        hash[1] += b;
        hash[2] += c;
        hash[3] += d;
        hash[4] += e;
        hash[5] += f;
        hash[6] += g;
        hash[7] += h;
    }
    hash
}

fn iteration_80(message_padding: &Vec<u8>, hash_init_values: &[u64; 8], k: &[u64; 80]) -> [u64; 8] {
    let mut w: [u64; 80] = [0; 80];
    let mut m: Vec<u64> = Vec::new();
    let mut hash = hash_init_values.clone();

    for chunk in message_padding.chunks(64 / 8) {
        m.push(u64::from_be_bytes(chunk.try_into().unwrap()));
    }

    // main loop
    for mi in 0..((m.len() * 64) / 1024) {
        for i in 0..16 {
            w[i] = m[(mi * 16)..(mi * 16 + 16)][i];
        }
        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = s1 + w[i - 16] + s0 + w[i - 7];
        }

        let mut a = hash[0];
        let mut b = hash[1];
        let mut c = hash[2];
        let mut d = hash[3];
        let mut e = hash[4];
        let mut f = hash[5];
        let mut g = hash[6];
        let mut h = hash[7];

        for i in 0..80 {
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h + s1 + ch + k[i] + w[i];
            let t2 = s0 + maj;

            h = g;
            g = f;
            f = e;
            e = d + t1;
            d = c;
            c = b;
            b = a;
            a = t1 + t2;
        }

        hash[0] += a;
        hash[1] += b;
        hash[2] += c;
        hash[3] += d;
        hash[4] += e;
        hash[5] += f;
        hash[6] += g;
        hash[7] += h;
    }
    hash
}
#[cfg(test)]
mod hash_test {
    use super::{sha, ShaType};
    #[test]
    fn hash() {
        let query = &String::from("sha256");

        let sha_type = match query.as_str() {
            "sha224" => ShaType::SHA224,
            "sha256" => ShaType::SHA256,
            "sha384" => ShaType::SHA384,
            "sha512" => ShaType::SHA512,
            _ => ShaType::SHA256,
        };
        let message: String = String::from("SUNYSUNYSUNYSUNY");
        assert_eq!(
            sha(&message, sha_type),
            String::from("142ea313267fe7670d878726214c30b6850a1e189edeff9cd4f769ba02371180")
        );
    }
}
