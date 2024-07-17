use std::ops::Index;

const S_BOX: [u8; 16] = [
    0b1110, 0b0100, 0b1101, 0b0001,
    0b0010, 0b1111, 0b1011, 0b1000,
    0b0011, 0b1010, 0b0110, 0b1100,
    0b0101, 0b1001, 0b0000, 0b0111,
];

fn s_block(input: u8) -> u8 {
    let l = (input >> 4) & 0x0F;
    let r = input & 0x0F;
    let result = (S_BOX[l as usize] <<  4 ) | S_BOX[r as usize];
    result
}

fn s_block_inverse(input: u8) -> u8 {
    let l = (input >> 4) & 0x0F;
    let r = input & 0x0F;
    let result = (S_BOX.iter().position(|&x| x == l).unwrap() << 4) as u8 | S_BOX.iter().position(|&x| x == r).unwrap() as u8;
    result
}


fn p_block(input: u8) -> u8 {
    let mut output = 0;
    for i in 0..4 {
        output |= ((input >> i) & 0x01) << (7 - i);
        output |= ((input >> 4 + i) & 0x01) << (7 - i - 4);

    }
    output
}

fn p_block_inverse(input: u8) -> u8 {
    p_block(input)
}

fn main() {
    let input: u8 = 0b11001100;
    let s_output = s_block(input);
    let s_inverse_output = s_block_inverse(s_output);
    println!("S-block output: {:08b}, S-block inverse output: {:08b}", s_output, s_inverse_output);

    let p_output = p_block(input);
    let p_inverse_output = p_block_inverse(p_output);
    println!("P-block output: {:08b}, P-block inverse output: {:08b}", p_output, p_inverse_output);
}
