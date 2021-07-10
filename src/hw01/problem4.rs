/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    // Terminating condition. No more discs to move to destination.
    if num_discs == 0 {
        return vec![];
    }

    let mut moves = vec![];
    // We move n - 1 discs to auxiliary, so then we can move the last disc to the destination
    moves.append(&mut hanoi(num_discs - 1, src, dst, aux));

    // Moving the last disc to the destination
    moves.push((src, dst));

    // Moving the n - 1 discs in auxiliary to destination
    moves.append(&mut hanoi(num_discs - 1, aux, src, dst));
    return moves;
}