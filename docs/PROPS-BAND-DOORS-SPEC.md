# PROPS band-doors — the `# Errors` sentence made true, and the door four suites open-code

**Binding at dispatch** (PROPS program; items
`work/props/band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps.md`
and `work/props/band-has-no-door-for-an-explicit-eps-with-the-runs-k.md`
— read both in full, including the first's `## Corrected and widened by
PR 2378's full review`; difficulty logged at spec: **E**, an E rider —
single style review, outside the A/B experiment). Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/band-doors`, cut from `main`. Both items live in one file,
`crates/geom-core/src/predicate.rs`, which is why they are one unit.

## Item 1 — the `# Errors` sentence is false, and a caller was built on it

`Band::linear`'s `# Errors` says `BandError` arises **only** when K·ε
overflows. A second arm is reachable with nothing overflowing:
`Tolerance::validate` admits any finite ε > 0 and any finite K > 1, so
at the minimum subnormal ε = 5e-324 with K = 1 + 2⁻⁵² the increment
K·ε − ε rounds away, `K·ε == ε`, the band has `zero == escalate`, and
`BandError::Empty` is returned. `Band::angular_at` reaches the same arm
at an ORDINARY ε — see the item's amendment for its arithmetic, and
re-derive it rather than quoting it.

This is not pedantry about a doc: `SelectRefusal::Band` was a unit
variant, and the argument for discarding the `BandError` was this
sentence — one cause, so naming it adds nothing. **The two ends want
opposite repairs** (ε near `f64::MAX`: lower ε; subnormal ε with K near
1: raise one of them), so a refusal naming neither sends half its
readers the wrong way.

**Deliver**: both `# Errors` sentences made true, each arm named, the
honest "unreachable for any physically meaningful tolerance" KEPT for
the overflow arm because that part is still right; and rows in
`predicate.rs`'s own `mod tests` pinning both reachable arms, written
as assertions over the validator's invariants rather than over literals
(the `editor-core` end already has its version — cite it by name, do
not duplicate it). **Not owed, and do not do it**: a change to
`Tolerance::validate`. Whether it should admit subnormal ε is a
separate question this row does not ask; a doc that describes the
validator it has is the fix.

## Item 2 — the door, and the orchestrator's ruling on its shape

`Band::linear(tol)` derives ε from the run; `Band::from_zero_threshold`
is private; so a suite that pins its own ε but wants the run's
escalation behaviour has nothing to call and open-codes
`Band::new(eps, tol.k() * eps)`. Four sites do it:
`crates/sweep/tests/common/approx.rs` (`reattach_certifies_at`),
`crates/sweep/tests/sf2b_r1_probes.rs`,
`crates/geom-brep/tests/pcurve_p1b_r2_probes.rs:~477`,
`crates/sweep/tests/review_fillet_h6_r2_probes.rs:~72`.

The item leaves the shape open between a named constructor, making
`from_zero_threshold` public, and refusing the door and documenting the
inline spelling. **Ruled: the named constructor**, `Band::linear_at(eps,
tol)` or a better name you argue for. The reason is not that it saves a
multiplication — the item is right that it would not earn its name for
that — but that `tol.k() * eps` spells the K-COUPLING by hand at four
sites, and the finding that produced the parent item was literal K
where the run's K belongs. One door makes the coupling one thing.
`from_zero_threshold` stays private: it names a threshold, not the
run's K, and publishing it would leave the same coupling open-coded.

**Refute this ruling if the code refutes it** — if the four sites turn
out not to want one shape, say so with what they want instead; a
disagreed ruling with evidence is a finding, not a deviation.

**Deliver**: the door with its doc stating what it is for and how it
differs from `linear`; the four sites converted; a row pinning that the
door and the inline spelling agree; a grep for any fifth site.

## Posture

- ε posture: none — no tolerance read moves; the door reads the ε it is
  handed and the K off the witness. Say so.
- Bit identity: the four converted sites must produce the same band
  they produce today, which the agreement row pins.
- **This machine has a build mutex** (`local-scripts/with-build-slot.sh`,
  `memories/agent-lane-operations.md` §Build concurrency) and it is
  contended. Wrap every heavy cargo call, pass no `-j`, expect to lose
  the slot, and let hosted CI be the verification of record.
- Review: single style review, outside the experiment.
- **Landing: both items get `pr:` and `status: review`. DO NOT MERGE** —
  the orchestrator lands after the review and its fix pass, closes both
  items and deletes this spec at merge with its `## Per-merge deletion`
  section in `docs/DOC-LEDGER.md`. No `Co-Authored-By`, no `CI-Config:`
  trailer, no empty commits.

## Acceptance

Both `# Errors` sentences true with both arms named and pinned by rows
that rest on the validator's invariants; the door minted, documented,
and the four sites converted with an agreement row; hosted CI green.
