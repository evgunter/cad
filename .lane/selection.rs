
/// Resolves a fillet's edge selection against the target's name table
/// (M6-5). Single-operand, so simpler than
/// [`resolve_declarations`] — but the refusal vocabulary is the SAME
/// N5 trio, deliberately: the two sites answer the same question, and
/// they answer it through the same [`ladder`], which owns rung order
/// and payload shapes. What stays here is this door's arity — one
/// table — and its kind refusal: a selection names EDGES.
///
/// The returned keys are in TARGET-ARENA order, not selection order,
/// so the kernel sees the deterministic order every derived list in
/// this kernel inherits (D9) regardless of how the recipe sorted.
fn resolve_selection(
    selection: &[names::StableName],
    doc: &crate::doc::Doc<ProfileProgram>,
    target: &NameTable,
) -> Result<Vec<topo::EdgeKey>, NodeErrorKind> {
    use crate::names::EntityKey;

    if selection.is_empty() {
        return Err(NodeErrorKind::FilletSelectionEmpty);
    }
    let mut keys = Vec::with_capacity(selection.len());
    for name in selection {
        let refused = |error| NodeErrorKind::FilletSelectionResolve {
            error: Box::new(error),
        };
        let live = ladder::live(name, doc).map_err(refused)?;
        let ent = ladder::resolve(live, ladder::landing(target, name)).map_err(refused)?;
        let EntityKey::Edge(k) = ent.key else {
            return Err(NodeErrorKind::FilletSelectionKind {
                name: Box::new(name.clone()),
                found: ent.key.kind(),
            });
        };
        keys.push(k);
    }
    // D9 order; the kernel refuses a repeated edge itself, so a
    // duplicate that survived canonicalization still fails loudly.
    keys.sort_unstable();
    Ok(keys)
}
