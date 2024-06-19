///////////////////////////////////////////////////////////////////////////////

/// Pretty print current state
pub fn debug(o: &mut Vec<u32>, a: &mut Vec<u32>, t: &mut Vec<u32>, d: u32) {
    print!("{} | ", d);
    for _ in 0..d {
        print!("|  ");
    }
    println!("o: {:?} a: {:?} t: {:?}", o, a, t);
    check(o, a, t, d);
}

//---------------------------------------------------------------------------//

/// Panics if state is invalid
pub fn check(o: &mut Vec<u32>, a: &mut Vec<u32>, t: &mut Vec<u32>, d: u32) {
    if !o
        .iter()
        .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
        .0
    {
        println!("unsorted org");
        panic!("unsorted org")
    }
    if !a
        .iter()
        .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
        .0
    {
        println!("unsorted aux");
        panic!("unsorted aux")
    }
    if !t
        .iter()
        .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
        .0
    {
        println!("unsorted target");
        panic!("unsorted target")
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Recursively solves Hanoi's Tower for 3 pegs and variable disks
///
/// `m` is the number of disk to move
/// `o` is the origin vector
/// `a` is the auxiliary vector
/// `t` is the target vector
/// `d` is the depth (start at 0)
/// `de` is a function that pretty prints the current state and checks if its
///     valid (i.e. `debug`).
///
/// Side effects:
///     Move `m` disks from `o` to `t`
///
/// Time complexity:
///     O(2^m)
///
pub fn hanoi_simple_rec(
    m: usize,
    o: &mut Vec<u32>,
    a: &mut Vec<u32>,
    t: &mut Vec<u32>,
    d: u32,
    de: &dyn Fn(&mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>, u32),
) {
    // O(m)
    de(o, a, t, d);

    // O(1)
    if m == 0 {
        // no disks, do nothing
        return;
    } else if m == 1 {
        // O(1)
        // one disk, just move the disk to target
        t.push(o.pop().unwrap());
        de(o, a, t, d);
    } else if m == 2 {
        // O(1)
        // 2 disks
        // one disk to aux
        // one disk to target
        // previous disk from aux to target
        a.push(o.pop().unwrap());
        de(o, a, t, d);
        t.push(o.pop().unwrap());
        de(o, a, t, d);
        t.push(a.pop().unwrap());
        de(o, a, t, d);
    } else {
        // general case

        // O(n-m) but would be O(1) if using a stack
        // this is instead of only passing a section of `o` in recursion
        let last = o.remove(o.len() - m);

        // recursive depth of m-1
        // recursively move m-1 disks from origin to aux
        hanoi_simple_rec(m - 1, o, t, a, d + 1, &|o, a, t, d| {
            de(o, t, a, d);
        });

        // O(1)
        // move the last remaining peg from origin to target
        t.push(last);
        de(o, a, t, d);

        // recursive depth of m-1
        // recursively move the m-1 disks on aux over to target
        hanoi_simple_rec(m - 1, a, o, t, d + 1, &|o, a, t, d| {
            de(a, o, t, d);
        });
        de(o, a, t, d);

        /*
        If each level of recursion has two branches, and the depth is m-1,
        the total size should be 2^(m-1), so it should have an O(2^m)
         */
    }
}

//---------------------------------------------------------------------------//

/// Iteratively solves Hanoi's Tower for 3 pegs and variable disks
///
/// `o` is the origin vector
/// `a` is the auxiliary vector
/// `t` is the target vector
///
/// Side effects:
///     Move all disks from `o` to `t`
///
/// Time complexity:
///     O(2^n) where n is the number of disks
///
pub fn hanoi_simple_iter(o: &mut Vec<u32>, a: &mut Vec<u32>, t: &mut Vec<u32>) {
    // O(1)
    // Moves a legal disk from a to b or vice versa
    fn inner(a: &mut Vec<u32>, b: &mut Vec<u32>) {
        // O(1)
        // get top disks
        let vala = a.last();
        let valb = b.last();

        // O(1)
        match (vala, valb) {
            // both are empty
            (None, None) => {} // O(1)
            // one is empty, move to empty
            (None, Some(_)) => a.push(b.pop().unwrap()), // O(1)
            // one is empty, move to empty
            (Some(_), None) => b.push(a.pop().unwrap()), // O(1)
            // neither is empty, move the smallest
            (Some(vala), Some(valb)) => {
                if vala < valb {
                    b.push(a.pop().unwrap()); // O(1)
                } else {
                    a.push(b.pop().unwrap()); // O(1)
                }
            }
        }
    }

    while (!o.is_empty()) || (!a.is_empty()) {
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);

        // O(1)
        // moves a disk from origin to aux or vice versa
        inner(o, a);
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);

        // O(1)
        // moves a disk from origin to target or vice versa
        inner(o, t);
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);

        // O(1)
        // move a disk from aux to target or vice versa
        inner(a, t);
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);
    }

    /*
    The while loop moves disks in a similar form to the recursive method.
    The details are explained on Wikipedia, but the general idea is that
    solutions all contain repeating patterns.

    The primary pattern is that we move a legal disk on alternating pegs.
    To be completely honest, I'm not sure how to clearly demostrate
    this is the optimal solution, but I don't really have time to try to
    create a graphic or proof.

    All this means is that this takes the same number of moves as a recursive
    approach, 2^n moves given n disks.
    */
}

///////////////////////////////////////////////////////////////////////////////

/// Pretty prints the current state (for general case problems)
///
/// Panics on invalid states
fn debug2(m: usize, d: u32, pegs: &Vec<Vec<u32>>, source: usize, target: usize) {
    print!("{} | ", d);
    for _ in 0..d {
        print!("|  ");
    }
    for peg in pegs {
        print!("{:?}", peg);
    }
    print!(
        "   ( s: {}{:?} t: {}{:?} )",
        source, pegs[source], target, pegs[target]
    );
    println!();

    for peg in pegs {
        if !peg
            .iter()
            .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
            .0
        {
            println!("unsorted peg");
            panic!("unsorted peg")
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Recursively solves Hanoi's Tower for any arrangement of pegs and disks.
///
/// `m` is the number of disks to move
/// `d` is the depth (start at 0)
/// `pegs` is the list of all pegs
///     each peg is a `u32` vector
/// `source` is the index for the origin peg in `pegs`
/// `target` is the index for the target peg in `pegs`
/// `aux` is the list of indices for auxiliary pegs in `pegs`
/// `moves` is the list of moves required to solve Hanoi's Tower
///     (start with an empty list)
///
/// Side effects:
///     Moves `pegs[source]` to `pegs[target]`
///
/// Input limits:
///     `aux` must not be empty
///     `pegs` must have 3 or more entries
///
pub fn hanoi_general_rec(
    m: usize,
    d: u32,
    pegs: &mut Vec<Vec<u32>>,
    source: usize,
    target: usize,
    aux: Vec<usize>,
    moves: &mut Vec<(usize, usize, u32)>,
) {
    assert!(pegs.len() >= 3);

    debug2(m, d, pegs, source, target);

    // O(1)
    if m == 0 {
        // no disks, do nothing
        return;
    } else if m == 1 {
        // O(1)
        // one disk, move directly to target
        let val = pegs[source].pop().unwrap();
        pegs[target].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((source, target, val));
    } else if m == 2 {
        // O(1)
        // two disk, same process as in 3 peg recursive
        // one disk to aux
        let val = pegs[source].pop().unwrap();
        pegs[aux[0]].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((source, aux[0], val));

        // O(1)
        // other disk to target
        let val = pegs[source].pop().unwrap();
        pegs[target].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((source, target, val));

        // O(1)
        // aux disk to target
        let val = pegs[aux[0]].pop().unwrap();
        pegs[target].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((aux[0], target, val));
    } else {
        // general case

        // O(1)
        let k = aux.len();

        // check if there are more auxiliary pegs than there are disks
        if m <= k + 1 {
            // O(m)
            // if so, we just move each disk to its own peg, then collect them
            // all, one by one, onto the target.
            println!("extra pegs");
            for i in 0..m - 1 {
                let val = pegs[source].pop().unwrap();
                pegs[aux[i]].push(val);
                debug2(m, d, pegs, source, target);
                moves.push((source, aux[i], val));
            }
            let val = pegs[source].pop().unwrap();
            pegs[target].push(val);
            moves.push((source, target, val));
            for i in (0..m - 1).rev() {
                let val = pegs[aux[i]].pop().unwrap();
                pegs[target].push(val);
                debug2(m, d, pegs, source, target);
                moves.push((aux[i], target, val));
            }
        } else {
            // keep track of bottom disks
            let mut vals = vec![];

            // O(n-m) but would be O(1) if using a stack
            // get the bottom-most disk
            let bottom = pegs[source].len() - m;
            let val = pegs[source].remove(bottom);

            // set aside one aux peg for the standard (3 peg) case
            // we can spread out disks on the remaining pegs
            // for now, we're just removing them from the bottom for later
            for i in 1..k {
                if pegs[source].is_empty() {
                    break;
                } else {
                    let val = pegs[source].remove(bottom);
                    vals.push(val);
                }
            }

            // recursive depth of m-k
            // solve standard (3 peg) case for the remaining disks
            // move them all to the first auxiliary peg
            hanoi_general_rec(m - k, d + 1, pegs, source, aux[0], vec![target], moves);
            debug2(m, d, pegs, source, target);

            // O(k)
            // spread out the bottom pegs we saved earlier
            // each disk gets its own peg
            for i in 1..k {
                if vals.is_empty() {
                    break;
                } else {
                    let val = vals.pop().unwrap();
                    pegs[aux[i]].push(val);
                    debug2(m, d, pegs, source, target);
                    moves.push((source, aux[i], val));
                }
            }

            // O(1)
            // move the now exposed bottom most peg directly to target
            pegs[target].push(val);
            debug2(m, d, pegs, source, target);
            moves.push((source, target, val));

            // O(k)
            // now, in reverse order, collect the spread out pegs onto target
            for i in (1..k).rev() {
                if pegs[aux[i]].is_empty() {
                    break;
                } else {
                    let val = pegs[aux[i]].pop().unwrap();
                    pegs[target].push(val);
                    debug2(m, d, pegs, source, target);
                    moves.push((aux[i], target, val));
                }
            }

            // recursive depth of m-k
            // finally, solve the standard 3 peg case, moving the disks
            // on the first aux peg to the target peg
            hanoi_general_rec(m - k, d + 1, pegs, aux[0], target, vec![source], moves);

            // we only expose one auxiliary peg to subsequent calls to avoid
            // weird cases where pegs are assumed to be empty but aren't

            // we explicitly track m to avoid a weird bug that I still don't
            // totally understand but comes up when the solver is only supposed
            // to move the top most couple items.

            /*
            Similarly to the 3 peg version, worst case levels of recursion will
            have two branches. However, here, the depth is m-k. This would
            suggest we would get 2^(m-k) moves. However, the example provided
            by geeksforgeeks and wikipedia both suggest much much lower values.
            I don't have time to look into this further, but I'm not sure
            what the total should end up at. I imagine 2^(m-k) is at least a
            safe upper bound.
            */
        }
    }

    debug2(m, d, pegs, source, target);
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi_simple_rec() {
        for i in 0..20 {
            let f: Vec<u32> = (0..i).rev().collect();
            let mut o = f.clone();
            let mut a = vec![];
            let mut t = vec![];
            hanoi_simple_rec(
                i.try_into().unwrap(),
                &mut o,
                &mut a,
                &mut t,
                0,
                &|o, a, t, d| debug(o, a, t, d),
            );
            assert_eq!(o, vec![]);
            assert_eq!(a, vec![]);
            assert_eq!(t, f.clone());
            println!();
        }
    }

    #[test]
    fn test_hanoi_simple_iter() {
        for i in 0..20 {
            let f: Vec<u32> = (0..i).rev().collect();
            let mut o = f.clone();
            let mut a = vec![];
            let mut t = vec![];
            hanoi_simple_iter(&mut o, &mut a, &mut t);
            assert_eq!(o, vec![]);
            assert_eq!(a, vec![]);
            assert_eq!(t, f.clone());
            println!();
        }
    }

    #[test]
    fn test_hanoi_rec() {
        for i in 3..20 {
            for j in 0..20 {
                println!("{} pegs with {} disks", i, j);
                let f: Vec<u32> = (0..j).rev().collect();
                let mut rods = vec![];
                for n in 0..i {
                    rods.push(vec![]);
                }
                rods[0] = f.clone();
                hanoi_general_rec(
                    j.try_into().unwrap(),
                    0,
                    &mut rods,
                    0,
                    i - 1,
                    (1..i - 1).collect(),
                    &mut vec![],
                );
                for n in 0..i - 1 {
                    assert_eq!(rods[n], vec![]);
                }
                assert_eq!(rods[i - 1], f.clone());
                println!();
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
