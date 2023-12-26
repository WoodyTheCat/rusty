use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::types::bitboard::ToBitboard;
use crate::types::bitboard::BB;
use crate::types::square::Square;

use super::MagicEntry;

// --------

include!("magic_numbers.rs");

// --------

fn magic_index(entry: &MagicEntry, blockers: BB) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

fn slider_moves(slider_deltas: &[(i8, i8)], square: SquareIndex, blockers: BB) -> BB {
    let mut moves = BB::EMPTY;
    for &(df, dr) in slider_deltas {
        let mut ray = square;
        while !blockers.has(ray) {
            if let Some(shifted) = ray.try_offset(df, dr) {
                ray = shifted;
                moves |= ray.to_bitboard();
            } else {
                break;
            }
        }
    }
    moves
}

fn make_table(
    table_size: usize,
    slider_deltas: &[(i8, i8)],
    magics: &[MagicEntry; Square::NUM],
) -> Vec<BB> {
    let mut table = vec![BB::EMPTY; table_size];
    for &square in &Square::ALL {
        let magic_entry = &magics[square as usize];
        let mask = BB(magic_entry.mask);

        let mut blockers = BB::EMPTY;
        loop {
            let moves = slider_moves(slider_deltas, square, blockers);
            table[magic_index(magic_entry, blockers)] = moves;

            // Carry-Rippler trick that enumerates all subsets of the mask, getting us all blockers.
            // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set#All_Subsets_of_any_Set
            blockers.0 = blockers.0.wrapping_sub(mask.0) & mask.0;
            if blockers.is_empty() {
                break;
            }
        }
    }
    table
}

fn write_table(name: &str, table: &[BB], out: &mut impl Write) -> std::io::Result<()> {
    write!(out, "const {}_MOVES: &[u64; {}] = &[", name, table.len())?;
    for entry in table {
        write!(out, "{},", entry.0)?;
    }
    write!(out, "];\n")?;
    Ok(())
}

fn write_magics(
    name: &str,
    magics: &[MagicEntry; Square::NUM],
    out: &mut impl Write,
) -> std::io::Result<()> {
    write!(
        out,
        "const {}_MAGICS: &[MagicEntry; Square::NUM] = &[",
        name
    )?;
    for entry in magics {
        write!(
            out,
            "MagicEntry {{ mask: {}, magic: {}, shift: {}, offset: {} }},",
            entry.mask, entry.magic, entry.shift, entry.offset
        )?;
    }
    write!(out, "];\n")?;
    Ok(())
}

pub fn main() {
    let rook_table = make_table(
        ROOK_TABLE_SIZE,
        &[(1, 0), (0, -1), (-1, 0), (0, 1)],
        ROOK_MAGICS,
    );
    let bishop_table = make_table(
        BISHOP_TABLE_SIZE,
        &[(1, 1), (1, -1), (-1, -1), (-1, 1)],
        BISHOP_MAGICS,
    );

    let mut out: PathBuf = PathBuf::from(r"C:\Users\System Owner\chess_temp\");
    out.push("magics.rs");
    let mut out = BufWriter::new(File::create(out).unwrap());

    write_magics("ROOK", ROOK_MAGICS, &mut out).unwrap();
    write_magics("BISHOP", BISHOP_MAGICS, &mut out).unwrap();
    write_table("ROOK", &rook_table, &mut out).unwrap();
    write_table("BISHOP", &bishop_table, &mut out).unwrap();
}
