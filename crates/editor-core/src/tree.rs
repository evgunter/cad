//! **The walks over an expression tree that keep their own stack.**
//!
//! [`Expr`](crate::Expr) and [`MeasureExpr`](crate::MeasureExpr) are
//! both trees of one- and two-operand nodes over leaves. The walks over
//! them that must cost the thread's stack nothing, however deep a tree
//! nests, are written once here for either: [`fold`], the evaluation
//! both evaluators run, and [`free`], the drop both types run.

/// How an evaluation walk values a node it visits.
pub(crate) enum Visit<'t, N, T> {
    /// Valued as it stands: a leaf, or a node valued by a walk of its
    /// own.
    Value(T),
    /// Valued from its one child's value.
    One(&'t N),
    /// Valued from its two children's values, first child first.
    Two(&'t N, &'t N),
}

/// The values a node's children left, in child order.
pub(crate) enum Operands<T> {
    /// A one-operand node's.
    One(T),
    /// A two-operand node's.
    Two(T, T),
}

/// One step of [`fold`]: visit a node, or combine the values its
/// children left (`binary`: two of them).
enum Step<'t, N> {
    Visit(&'t N),
    Combine { node: &'t N, binary: bool },
}

/// Values `root` from the bottom up on a heap stack: `visit` says how a
/// node is valued, and `combine` values a node from its children's
/// values.
///
/// Nodes are met in the order a recursive evaluation meets them — each
/// child, first child first, is valued before its parent combines — so
/// the same operations run in the same order and the first refusal is
/// the one a recursive evaluation would raise.
pub(crate) fn fold<'t, N, T, E>(
    root: &'t N,
    mut visit: impl FnMut(&'t N) -> Result<Visit<'t, N, T>, E>,
    mut combine: impl FnMut(&'t N, Operands<T>) -> Result<T, E>,
) -> Result<T, E> {
    let mut work = vec![Step::Visit(root)];
    let mut values = Vec::new();
    while let Some(step) = work.pop() {
        match step {
            Step::Visit(node) => match visit(node)? {
                Visit::Value(value) => values.push(value),
                Visit::One(a) => {
                    work.push(Step::Combine {
                        node,
                        binary: false,
                    });
                    work.push(Step::Visit(a));
                }
                Visit::Two(a, b) => {
                    work.push(Step::Combine { node, binary: true });
                    work.push(Step::Visit(b));
                    work.push(Step::Visit(a));
                }
            },
            Step::Combine { node, binary } => {
                let last = left(&mut values);
                let operands = if binary {
                    Operands::Two(left(&mut values), last)
                } else {
                    Operands::One(last)
                };
                values.push(combine(node, operands)?);
            }
        }
    }
    Ok(left(&mut values))
}

/// The value the last node valued left.
fn left<T>(values: &mut Vec<T>) -> T {
    match values.pop() {
        Some(value) => value,
        None => unreachable!(
            "a walk combined a node with no value left, yet every child is valued, and leaves \
             its value, before its parent combines"
        ),
    }
}

/// Frees the tree `node` roots from a heap stack: `detach` moves a
/// node's children onto the stack and leaves it a leaf, so every node
/// is dropped with no children to recurse into.
pub(crate) fn free<N>(node: &mut N, detach: impl Fn(&mut N, &mut Vec<N>)) {
    let mut stack = Vec::new();
    detach(node, &mut stack);
    while let Some(mut child) = stack.pop() {
        detach(&mut child, &mut stack);
    }
}

/// `levels` levels of `wrap` over `leaf`: a chain built past the
/// constructors, which refuse it past the bound, for the rows that
/// measure the walks here on trees far deeper than a document holds.
#[cfg(test)]
pub(crate) fn raw_chain<N>(leaf: N, levels: usize, wrap: impl Fn(N) -> N) -> N {
    (1..levels).fold(leaf, |node, _| wrap(node))
}
