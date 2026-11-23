//! **A decided insertion sort**, for orders read through the band.

/// `xs` sorted by `after(a, b)`, whether `a` stands strictly after `b`,
/// asked of each element against its predecessors until one does not.
/// Stable: elements no verdict puts apart keep their insertion order.
/// The first `Err` stops the sort and comes back.
///
/// # Errors
///
/// `after`'s first.
pub fn insertion_sort_by<X, E>(
    xs: &mut [X],
    mut after: impl FnMut(&X, &X) -> Result<bool, E>,
) -> Result<(), E> {
    for i in 1..xs.len() {
        let mut j = i;
        while j > 0 && after(&xs[j - 1], &xs[j])? {
            xs.swap(j - 1, j);
            j -= 1;
        }
    }
    Ok(())
}
