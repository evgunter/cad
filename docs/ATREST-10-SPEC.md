# ATREST-10 — the at-rest door matrix

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-10.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.
**Dispatched after ATREST-3 (PR #3191) lands** — it changed what the
certificate doors return and where check 7 is made.

Branch `atrest/10-door-matrix`. Rows carried:
`work/atrest/structural-suffix-means-two-things-across-the-six-doors`,
`work/atrest/validate-rs-exports-sixteen-doors-on-an-irregular-matrix`.
Read both, and `work/scalar/H5.md` §RATIFIED ruling 3 — Ev's ruling
that a `_structural` twin is the door holding no certified lane, at any
scalar.

## Settled design

**D-A. `_structural` means one thing: check 7 is made, through the
closed form, and no certified lane is held.** Four of the six
`_structural` doors already do this; `validate_geometric_structural`
(and `_structural_declared`) make no check 7 at all. After ATREST-3,
check 7 is made in exactly one place — `plus_v_by_sign` — and a lane of
`None` IS the closed-form derivation, so the composed door's old
argument ("the sign is decided in exactly one place") now argues FOR
making it, not against. Make `validate_geometric_structural` call
`plus_v_by_sign` with no lane, as its siblings do. This is within
ruling 3's letter (no certified lane is held); if you find ratified
text it contradicts (`git log --all -S'<phrase>'` per CLAUDE.md), stop
and report — that would be an `[ev]` question. `topo/tests/geometric_cube.rs`'s
`the_structural_half_does_not_judge_orientation_at_any_scalar` moves
with the choice (discipline §3: re-baseline, say what moved).

**D-B. Draw the roster deliberately.** Tabulate every public at-rest
door as {`validate_geometric`, `validate_pseudomanifold`,
`contact_marks`} × forms. Rules: every certified door has a
`_structural` twin (ruling 3); a form that exists for ONE door only
either gains a one-line reason at that door or its siblings gain it or
it goes — decide per hole and give the reason in the PR body. Do not
split `validate.rs` in this unit; if the table shows the split is
warranted, file it as its own row.

**D-C. The roster becomes a checked table.** Put the door table in
`validate.rs`'s module doc and add a test that reads the crate's public
exports and fails when a door exists that the table lacks, or the table
names a door that does not exist — so the next rename cannot grow a
hole. Say what the census cannot see.

## Public surface

Renaming or removing a public door is carried through every cargo root
(`scripts/doc-gate.sh --print-roots`), `demos/tour`, `demos/wild`,
`pncad-py` and its stubs and census, and every prose citer. Prefer
adding a missing form over removing a used one; removing a door with an
outside caller needs its reason in the PR body.

## What you owe

The change; the roster table; the census row; the re-baselined rows;
the sweep per discipline §5; out-of-fence findings filed. Set the two
carried rows to `review` when you open the PR.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live; never hold the build slot for a battery. Do not touch
`work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: public API shape across crates, but the design is
settled here from a ratified ruling and ATREST-3's single derivation;
what remains is execution and the census. Class M / STRUCTURAL.
