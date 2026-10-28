pub fn most_significant_bit(x: u8) -> bool {
    x & 0b10000000 != 0
}

pub fn toggle_bit(x: u8, n: u8) -> u8 {
    x ^ (1u8 << n)
}

pub fn xtime(a: u8) -> u8 {
    if most_significant_bit(a) {
        (a << 1) ^ 0x1B
    } else {
        a << 1
    }
}

pub fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        b >>= 1;
        a = xtime(a);
    }
    result
}

pub fn gf_pow(mut a: u8, mut n: u8) -> u8 {
    let mut result = 1;
    while n > 0 {
        if n & 1 != 0 {
            result = gf_mul(result, a);
        }
        a = gf_mul(a, a);
        n >>= 1;
    }
    result
}

pub fn gf_inv(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }
    gf_pow(a, 254)
}

   fn sbox_value(b: u8) -> u8 {
       let inv = gf_inv(b);         
       let mut result = 0u8;

       for i in 0..8 {        
           let bit_i     = (inv >> i) & 1;
           let bit_i4    = (inv >> ((i + 4) % 8)) & 1;
           let bit_i5    = (inv >> ((i + 5) % 8)) & 1;
           let bit_i6    = (inv >> ((i + 6) % 8)) & 1;
           let bit_i7    = (inv >> ((i + 7) % 8)) & 1;

 
           let bit_c     = (0x63 >> i) & 1;

        
           let out_bit   = bit_i ^ bit_i4 ^ bit_i5 ^ bit_i6 ^ bit_i7 ^ bit_c;

          
           if out_bit == 1 {
               result |= 1 << i;
           }
       }

       result
   }

fn generate_sbox() -> [u8; 256] {
    let mut sbox = [0u8; 256];
    for i in 0..256 {
        sbox[i] = sbox_value(i);
    }
    sbox

}




#[test]
fn test_msb() {
    assert_eq!(most_significant_bit(128), true);
    assert_eq!(most_significant_bit(127), false);
    assert_eq!(toggle_bit(0, 0), 1u8);
    assert_eq!(toggle_bit(0b11111111, 7), 0b01111111);
    assert_eq!(toggle_bit(0b10101010, 3), 0b10100010);
    assert_eq!(xtime(0x05), 0x0A);
    assert_eq!(xtime(0x80), 0x1B);
    assert_eq!(gf_mul(0x57, 0x83), 0xC1);
    assert_eq!(gf_inv(0x00), 0x00);
    assert_eq!(gf_mul(0x57, gf_inv(0x57)), 1);
    assert_eq!(gf_pow(0x02, 3), xtime(xtime(0x02)));
}
