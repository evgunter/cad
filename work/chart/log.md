# CHART log

## Opened at TRIM's cut (2026-09-20)

Opened by TRIM's orchestrator on Ev's in-chat direction (break the two
programs into smaller tracks; leave in TRIM only what closes this
session). Thirty-one items moved from `work/trim/` and three SSI
drive-bys from `work/curved/`. Band 6200–6299 recorded in the ledger's
banding entry in the same commit. No unit dispatched; the first sitting
picks from `work/chart/plan.md` §Order. Context the items carry:
`min-separation-tightening-crosses-the-drive` was REWRITTEN by TRIM-3
PR-2's fix pass to the measured mechanism (both reviewers refuted the
original: the diverging census rows are the loop walk's, recorded by
the interval leaf and never by the f64 lane, which returns `None`);
`revolved-bands-reach-no-clearance-row` REPLACES a refuted file (an
extrude CAN mint a negative cylinder band); the rational-gate class's
`mesh` copy was fixed by TRIM-2 PR-2 (constancy), the cap class remains.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `normalize-without-the-length-question-two-more-sites`,
and it carries an AMENDMENT that contradicts its own first half.**

Site 1 is `crates/topo/src/chart_region.rs` (yours): the collinear lane
decides an offset whose norm is in the DENOMINATOR, and its `Sign::Zero`
arm then normalizes `r` — whose length no door asked to be finite.

**A reviewer EXECUTED it, and the mechanism the row argued is not the one
that fires.** What reproduces, through `proper_crossings` with two-point
loops: two collinear overlapping segments give `Err(TouchingBoundary)` at
finite scale, and the same configuration scaled by 1e199 gives **`Ok(0
crossings)`** — a touching boundary silently lost. Not the spurious `Zero`
the row predicted: the offset rung decides a CORRECT `Zero`, and then
`rhat = r.normalize()` is `(0, 0)`, which drives `s0 = s1 = 0`, so the
overlap decides `Zero` and the lane `continue`s past the crossing.
**The operative consequence: a fix at the offset rung would not close this
row.** The repair belongs where the collapsed `rhat` is USED. Start from
the reproduction, not from the argument above it.

**Site 2 travels with it and is not yours**:
`crates/geom-brep/src/enters.rs` is in no open program's `paths` —
territory returns no owner. `enters_material` normalizes a `dir` whose own
length is never decided or asked about, so a `dir` past the overflow band
normalizes to zero and the door answers `Tangent` — definite, and about
nothing. `geom_core::is_finite_length`'s docs call this *the declined half
of the direction family* and cite ONE instance; this is a second, so the
doc's count is wrong today whatever is decided about the half. **If you
would rather not carry an unowned geom-brep site, split it into its own
file rather than leaving it as a sentence** — `work/README.md` is explicit
that a residue disclosed in prose is invisible to the re-homing sweep.

Signed (FIX orchestrator).

## 2026-09-27 — seam note from S-DUP (#3304)

#3304 retyped topo's two ray-direction tables. `splitting::containment::SCHEDULE` is now `[Vec3<f64>; 16]`, and `chart_region::SCHEDULE_2D` is now `[Vec2<f64>; 16]`.

Their five lift sites now read `r.map(T::from_f64)`:
- `containment`, `order` and `solid_contain` (3-D table);
- `chart_region` and `chart_bound` (2-D table).

The numerals are textually unchanged. A throwaway test asserted every lifted component bit-identical at `f64` and `Interval` (160 components), and a one-ulp plant reddened it. No behaviour changed.

Signed (S-DUP orchestrator).
- 2026-09-29 — Seam note from ENCL: PR 3398 (merged `3aac59af62`) moves `topo::validate`'s remaining 'lower the tolerance' endings onto their decisions (D4 ¶1). `geom_brep::recourse` gains `Unsized` and `defect_ending`, moved out of certify. Filed: `pcurve-mint-escalation-carries-no-decision` (validate's `M::Escalated` ends 'no way through yet' until the mint carries its decision). (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/chart_region.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
