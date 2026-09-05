# TOPO-S330 — tier-3 check 1's described-NURBS arm (spec)

Item `work/topo/S330.md`, TOPO plan opener; rider `work/topo/S94.md`
(the two hand-kept `VARIANTS` ladders — the code-quality plan folds it
into the first lane that opens `validate.rs`, and this is that lane).
Branch `topo/s330-described-nurbs-arm`. **Difficulty pre-logged S/M;
task class STRUCTURAL.** Survey run 2026-09-05 against main
`18ea0915`; every line cite below was re-derived on that head.

**One sentence.** Tier-3 check 1 has three states to answer for a
`Surface::Nurbs` payload — the mvfs PLACEHOLDER (refused,
`UncertifiableSurface`), a DESCRIBED net of finite data (real geometry,
passes), and a DESCRIBED net carrying poison in some channel (corrupt
described geometry, which `geom`'s totality rule says must fail at every
consumer's described arm) — and today the third falls through `_ => {}`
and is answered by nothing at check 1. This unit adds the arm, through
the one poison door `geom` already spells, and leaves no wildcard in the
match.

---

## The claim

`crates/topo/src/validate.rs:2961-3034`, `tier3_local_checks_marked`'s
check 1: the match on `body.surfaces.get(face.surface)` has an arm
`Some(Surface::Nurbs(payload)) if payload.is_placeholder()` → 
`UncertifiableSurface`, arms for `Approx` and `Torus`, and `_ => {}` at
`:3034`. A described net whose every control point carries a poisoned
`x` over finite `y`/`z` is NOT the placeholder since S99
(`crates/geom/src/net.rs:135`, all channels of all points) and so reaches
the wildcard. `crates/geom/src/lib.rs:66-107` states the rule such a net
must meet: *"a poisoned CHANNEL of one point is corrupt described
geometry"* that *"must reach each consumer's described arm and fail
there"*. Check 1 is that consumer for surfaces and has no described arm.

The row's own account — the door still refuses the body, through check
2's edge re-certification, a different question answered by accident —
is a claim about the tree, and Phase 1 measures it before anything is
built on it.

## Phase 1 — measure (commit the probe with its output in the PR body)

1. Build the fixture: a tier-3-reachable body with ONE face carrying the
   masquerading net. `crates/topo/src/fixtures.rs:760` `ops_cube` is a
   tier-3-clean planar cube; `Body::set_face_surface`
   (`crates/topo/src/attach.rs:68`) swaps one face's surface; the net is
   `crates/geom/tests/net_placeholder_width.rs:68`'s
   `masquerading_surface` (bilinear, `x` poisoned at every point), which
   `crates/topo/src/n2r1_probes.rs:22` already spells inside `topo`.
2. Run `validate_geometric` on it at `f64` and record EVERY variant in
   the refusal, in order. That list is the "answered by accident"
   evidence: which checks fire (the expectation is check 2's
   `DescriptionNotAdjacent` on the face's four edges, since the swapped
   face's edges still describe the old plane — verify, do not assume)
   and that no check-1 variant appears.
3. Same body, same face, a FINITE described net (the same knots and
   weights with finite control points): record the refusal again. This
   is the control the acceptance row pins: check 1 must stay silent
   here after the change too.
4. Record what check 2 answers for a poisoned CURVE net on an edge
   (`recertify_nurbs_lane`, `crates/geom-brep/src/certify.rs:841`) —
   measurement only, one paragraph in the PR body; this unit does not
   touch curves.

If step 2 shows a check-1 variant already firing, stop and report: the
premise is wrong and the orchestrator re-scopes.

## Phase 2 — the change

**(a) The poison door, public.** `crates/geom/src/net.rs:154`
`any_poison` is `pub(crate)`; `NurbsSurface` exposes `is_placeholder`
(`crates/geom/src/surfaces/nurbs.rs:302`) and no complement. Add ONE
public method on `NurbsSurface` delegating to `net::any_poison` over
`self.control()`. Its doc states the three-state table (placeholder ⇒
`true`; described finite ⇒ `false`; described with poison anywhere ⇒
`true`) and points at the crate-doc rule. **This file is S-CERT's
territory**; the seam is announced on `work/cert/log.md` (2026-09-05)
for exactly this one method. Nothing else in `geom` moves. Do NOT
re-derive the channel walk in `topo` — the rule is spelled once, in
`net.rs`, and a second spelling is the smell Q1 of the style brief
exists to catch.

**(b) The arm, and no wildcard.** In check 1, in this order:

- `Some(Surface::Nurbs(p)) if p.is_placeholder()` — unchanged.
- `Some(Surface::Nurbs(p)) if p.<door>()` — push a NEW
  `ValidationError` variant carrying `face`. Name it for the state
  ("a described net carrying poison"), not for the symptom; its doc
  says how it differs from `UncertifiableSurface` (placeholder: no
  description yet, benign mid-surgery; this: a description that is
  corrupt) and its `Display` names the face and the state.
- `Some(Surface::Nurbs(_)) => {}` — the finite described net, with the
  one-line reason already in the check's header comment (real
  geometry; seams certify at check 2, flux at check 7).
- `Some(Surface::Approx(..))`, `Some(Surface::Torus {..})` — unchanged.
- Every remaining `Surface` variant (`crates/geom/src/surfaces.rs:91`)
  as a NAMED no-op arm with a one-line reason each. If, placing one,
  you find a datum check 1 should be making and is not (a cylinder or
  sphere radius, a cone half-angle — the `NonpositiveTorusTube` shape
  at `:3001-3014`), that is a FINDING for the PR body and the report,
  not a fix in this unit.
- `None => {}` with check 2's cascade-discipline sentence
  (`:3038-3042`): tier 1 already named the dangling surface.

No `_` arm survives. Update the check-1 rustdoc list
(`validate.rs:2181-2189`) and the `UncertifiableSurface` doc
(`:445-458`, which today describes two states) to state three.

**(c) The S94 rider.** `validate.rs:5945-5951` and
`crates/topo/src/euler.rs:3357-3365` each carry a hand-written
`const VARIANTS` and a wildcard-free `variant_index` restating the
enum's order, and the same four-sentence "what it does NOT enforce"
paragraph verbatim. The count and the index come from the compiler
after this unit, in ONE spelling for both files: `strum`'s `EnumCount`
plus `EnumDiscriminants` is the named route (the workspace already
builds `syn`, so a proc-macro derive costs no new toolchain; pick a
release at least two weeks old, `memories/review-and-dependency-policy.md`).
If you find an equal dependency-free spelling, take it and say why it
is equal. Both disclosure paragraphs go. `rg -n 'VARIANTS' crates/*/src`
for other ladders (`MassPropsError`, `PcurveMintError`, `BooleanError`
are the names S94 asks about) — the hit list and its disposition in
the PR body, fixes only inside this program's fence
(`work/topo/program.md` `paths`).

## Constraints, binding

1. `docs/prompts/implementer-discipline.md` in full, before anything.
2. Fence: `crates/topo/src/validate.rs`, `crates/topo/src/tier3_tests.rs`,
   `crates/topo/src/euler.rs` (the test module only), the ONE method in
   `crates/geom/src/surfaces/nurbs.rs`, `crates/topo/Cargo.toml` and
   `Cargo.lock` for the rider's dependency. Anything else is a report.
3. `crates/topo/src/pcurves.rs` and `census.rs` are other programs'
   (TRIM, CURVED); S331 and S350 live there and are not this unit.
4. Comments state the invariant. No row ids, no "S330", no history in
   code comments; the argument goes in the PR description.
5. Lane commits carry NO `Co-Authored-By` trailer and no model name. If
   one lands in a pushed commit, note it in the PR body and carry on —
   never rewrite history.
6. Merge `origin/main` before opening the PR and whenever main moves;
   after every push confirm the run's jobs are actually RUNNING (twelve
   `test (…)` jobs, five `k-lint (gate, …)`), and poll the run to its
   conclusion in the foreground before reporting.

## Acceptance

- **Red-first row** (`f64`), in `crates/topo/src/tier3_tests.rs`: the
  Phase-1 fixture refuses AND the refusal contains the new variant
  naming the swapped face. Record in the PR body that the row is red on
  the merge base (run it once against the arm-less build).
- **Control row**: the finite described net on the same face draws NO
  check-1 variant (neither `UncertifiableSurface` nor the new one),
  whatever check 2 says. This is what pins that check 1 itself moved.
- **Placeholder row** still answers `UncertifiableSurface`
  (`crates/topo/tests/geometric_cube.rs:133` is the existing pin — keep
  it green, cite it, do not duplicate it).
- **Interval row**: the red-first row at `T = Interval` under the
  `interval` feature (`crates/topo/tests/interval_body.rs` is the idiom;
  `crates/geom/tests/net_placeholder_width_interval.rs` is the poison at
  that scalar). If the fixture cannot be built there, the reason goes in
  the PR body with the line that stops it — not a silent omission.
- `errors_display_without_panicking` covers the new variant with a
  compiler-derived count, and `euler.rs`'s twin is on the same
  mechanism.
- Hosted CI green on the full matrix; no narrowing.
- PR body carries: the Phase-1 measurements (steps 2–4), the sweep
  receipt for `_ =>` arms over `Surface` matches in `validate.rs` (one
  hit at `:3034` is the expectation — list every hit and its
  disposition), the `VARIANTS` hit list, and any datum finding from (b).

## Out of scope

`validate_pcurves`' vacuous green (S331, TRIM's file); `face_reach`'s
partial box (S350, CURVED); a poisoned curve net at check 2 (measured,
reported, not fixed); datum checks for cylinder/sphere/cone (reported).

## Review

Protocol v6 dual on the frozen delivered head. Claims to falsify, for
both arms: **C1** the new arm fires on exactly the described-poisoned
state, never on the placeholder and never on a finite net; **C2** check
1 ITSELF now names the face — the control row, not check 2's findings,
is the evidence; **C3** no wildcard remains and every `Surface` variant
is placed with a true reason; **C4** the poison rule has one spelling
(`net.rs`) and `topo` calls it rather than re-deriving it; **C5** both
`VARIANTS` counts are compiler-derived and the disclosure text is gone;
**C6** the sweep receipts say what their patterns could not match.
