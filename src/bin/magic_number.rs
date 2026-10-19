use chess::bitboard::{BitBoard};
use chess::attack::{bishop_occupancy_mask_table, rook_occupancy_mask_table};

fn main() -> std::io::Result<()>{
    let mut rook_magics = [0u64; 64];
    let mut rook_offset = [0usize; 64];
    let mut rook_occ_attack_map: Vec<u64> = Vec::new();
    let mut r_offset = 0;

    let mut bishop_magics = [0u64; 64];
    let mut bishop_offset = [0usize; 64];
    let mut bishop_occ_attack_map: Vec<u64> = Vec::new();
    let mut b_offset = 0;

    for sq in 0..64 {
        rook_offset[sq] = r_offset;
        let Some((rn, rmap)) = find_magic(sq, &rook_occupancy_mask_table(), rook_attack_pattern) else { panic!("Rook Magic Number not found for {sq}") };
        rook_magics[sq] = rn;
        r_offset += rmap.len();
        rook_occ_attack_map.extend(rmap);

        bishop_offset[sq] = b_offset;
        let Some((bn, bmap)) = find_magic(sq, &bishop_occupancy_mask_table(), bishop_attack_pattern) else { panic!("Bishop Magic Number not found for {sq}") };
        bishop_magics[sq] = bn;
        b_offset += bmap.len();
        bishop_occ_attack_map.extend(bmap);
    }

    let mut output = String::new();
    output.push_str(&string_static_pub_rust_array_u64("ROOK_MAGICS", &rook_magics));
    output.push('\n');
    output.push_str(&string_static_pub_rust_array_usize("ROOK_OFFSET", &rook_offset));
    output.push('\n');
    output.push_str(&string_static_pub_rust_array_u64("ROOK_OCC_TABLE", &rook_occ_attack_map));
    output.push('\n');

    output.push_str(&string_static_pub_rust_array_u64("BISHOP_MAGICS", &bishop_magics));
    output.push('\n');
    output.push_str(&string_static_pub_rust_array_usize("BISHOP_OFFSET", &bishop_offset));
    output.push('\n');
    output.push_str(&string_static_pub_rust_array_u64("BISHOP_OCC_TABLE", &bishop_occ_attack_map));
    output.push('\n');

    std::fs::write("src/attack/generated_magics.rs", output)?;
    Ok(())
}

// Generate all occupancy (done)
// for each occupancy generate the attack set
// Test a magic number
// Check size of hash at the end 
// loop
//
//
// We can save the mask as well for each square (which we should do anyway)
// Apparently we should also save the shift amount (make sense that it might vary) (can also do
// plain which uses a fixed size shift)
//

// For each occupancy mask, generateall occupancy possibilities


// Get each position number
// Vector to build all the possibilities (2^number of squares)
// rebuild into a bitmap
// for each also generate the attack bitmap
//

//   000100
//   pos = 0
//   trailing zeros = 2
//
fn get_bitmask_positions(mask: BitBoard) -> Vec<usize> {
    let mut positions = Vec::with_capacity(mask.count_ones() as usize);
    let mut n = mask;
    let mut pos = 0;
    while n > 0 {
        let tz = n.trailing_zeros();
        positions.push((pos + tz) as usize);
        n >>= tz;
        n ^= 1u64;
        pos += tz;
    }
    positions
}

fn get_all_possible_occupancy(mask: BitBoard) -> Vec<BitBoard> {
    let positions = get_bitmask_positions(mask);
    let size : usize = 1usize << mask.count_ones();
    let mut bitboards = Vec::with_capacity(size);
    bitboards.push(0u64);

    for n in positions {
        for i in 0..bitboards.len() {
            bitboards.push(bitboards[i] | 1u64 << n);
        }
    }
    bitboards
}


// moves in this case is the relative transformation to apply
pub const fn generate_attack_pattern(moves: &[(i32, i32)], square: usize, occ: BitBoard) -> BitBoard {
    let mut attack_pattern = 0u64;
    let (rank, file) = ((square / 8) as i32, (square % 8) as i32);
    let mut km = 0;
    while km < moves.len() {
        let (i, j) = moves[km];
        let (mut curr_r, mut curr_f) = (rank + i, file + j);
        while curr_r >= 0 && curr_r < 8 && curr_f >= 0 && curr_f < 8 {
            attack_pattern |= 1u64 << (8 * curr_r + curr_f);
            
            if occ & 1u64 << (8 * curr_r + curr_f) != 0 { break; }
            curr_r += i;
            curr_f += j;
        }
        km += 1;
    }
    attack_pattern
}

fn rook_attack_pattern(square: usize, occ: BitBoard) -> BitBoard {
    let rook_moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    generate_attack_pattern(&rook_moves, square, occ)
}

fn bishop_attack_pattern(square: usize, occ: BitBoard) -> BitBoard {
    let bishop_moves = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    generate_attack_pattern(&bishop_moves, square, occ)
}


fn is_plain_magic_number(number: u64, bits: u32, occupancies: &[BitBoard], attacks: &[BitBoard]) -> Option<Vec<u64>> {
    if occupancies.len() != attacks.len() { return None; }
    let mut map = vec![None; 1usize << bits];
    for (&occ, &attack) in occupancies.iter().zip(attacks.iter()) {
        let hash = (number.wrapping_mul(occ) >> (64 - bits)) as usize; match map[hash] {
            Some(v) if v != attack => return None,
            None => map[hash] = Some(attack),
            _ => {}
        }
    }
    Some(map.into_iter().map(|item| item.unwrap_or(0u64)).collect())
}

fn find_magic(square: usize, occupancy_mask_table: &[u64], attack_fn: fn(usize, BitBoard) -> BitBoard) -> Option<(u64, Vec<u64>)> {
    let mask = occupancy_mask_table[square];
    let occupancies = get_all_possible_occupancy(mask);
    let attacks : Vec<BitBoard> = occupancies.iter().map(|&v| attack_fn(square, v)).collect();

    for _ in 0..100_000_000 {
        let n : u64 = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
        if let Some(map) = is_plain_magic_number(n, mask.count_ones(), &occupancies, &attacks) {
            return Some((n, map));
        }
    }
    None
}

fn string_static_pub_rust_array_u64(name: &str, content: &[u64]) -> String {
    let mut out = format!("pub static {name} : [u64; {len}] = [\n", len = content.len());
    for &item in content {
        out.push_str(&format!("    0x{item:0>16X},\n"));
    }
    out.push_str("];\n");
    out
}

fn string_static_pub_rust_array_usize(name: &str, content: &[usize]) -> String {
    let mut out = format!("pub static {name} : [usize; {len}] = [\n", len = content.len());
    for &item in content {
        out.push_str(&format!("    {item},\n"));
    }
    out.push_str("];\n");
    out
}
