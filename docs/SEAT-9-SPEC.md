# SEAT-9 — the shell arm on `Verb`, and ε stops travelling as an `f64` (unit spec)

Executes two rulings: VERB-SEAT-DESIGN §2 V4 for the shell (SEAT's
vocabulary arm — the enabler LIB-G17's `Node::Shell` waits on), and
Ev's ruling (i) on `[ev]` PR 1904 for
`work/seat/shell-doors-take-tolerance-beside-tol.md`: the shell doors'
raw `tolerance: f64` is dropped, the fit target is ε_precision, and
**the tolerance is passed as the ZST witness `Tol` all the way down —
never an `f64` in any signature between the shell door and the one
site that classifies the fit residual.** Faithful elaboration of two
rulings; self-merges. Any shape that reintroduces an `f64` epsilon in a
signature, or moves a funnel-site name, is a deviation — STOP and
report. This unit crosses territory (SHELL owns `topo/src/shell.rs`
and `replace_face.rs`; the offset fit is S-CERT's then PROPS'); it runs
`python3 scripts/work.py territory --base main` and puts the receipt in
the PR body — a warning, not a block, and the cross-program courtesy
is the log entry naming every file another program owns.

## S9-1 — the arm (`crates/verbs`)

`Verb::Shell { thickness: T, open: Vec<FaceKey> }` — arity One, the
body the operand; `run` dispatches to `topo::shell_open` (an empty
`open` IS `shell`, as the door already says). `VerbRecord::Shell(
ShellNaming)` carries the record `Shelled` mints by value, never
restated — a new variant is a compile-forced visit to every channel
consumer (state the visits). `VerbError` gains the shell's typed
refusal; `ShellError<T>` is generic in the scalar (it carries the
thickness), which is a structural decision for the closed error
enum — argue it (a `T` on `VerbError`, or the thickness rendered at
the door into a scalar-free refusal), never a stringly loss. A
second structural decision, measured by SEAT-8's dual: the shell doors
are bounded `Decide + PropsQuadLane + CertifiedBounds` — TIGHTER than
`Verb`'s impl header (`Decide + Bounds + PcurveFittedLane`,
`verbs/src/run.rs`), and `geom-core/src/real.rs` (the compound-`Bounds`
allowlist entry, ~line 503) records that tightening that header breaks
its `Dual` caller. So the shell arm cannot ride `run` as written: it
needs its own door under its own bound (a separately-bounded method or
impl block, the `run_profile` precedent for shape), with the mismatch
refusal still spoken by `Arity` at the other doors. Argue the shape;
the `Dual` caller stays green by construction (it never names the
shell door), and that is pinned. **No
content tag and no lowering**: `Node::Shell` does not exist (LIB-G17,
parked; its `blocked_on` int has fired — say so in the PR as courtesy).
The tag censuses over `VerbKind::ALL` must therefore learn, as closed
data, that a kernel-only verb has no document tag yet — an explicit
declaration (`document_tag: Option<u8>`-shaped, or a `KernelOnly`
census bucket), never a skipped arm. `param_flow`: explicit EMPTY
rows with the reason — `thickness` reaches only kernel-derived cavity
fields (`r − t`), which VS-Q3 gives no source in v1. The run-door
equivalence rows (`run_door.rs` precedent): body dump and every
`ShellNaming` field equal across `shell_open` vs `run`; refusals
cross with `Debug`/`Display` unchanged; the arity matrix extends.

## S9-2 — the tolerance, ruled

Drop `tolerance: f64` from `topo::shell` and `shell_open`. Thread `Tol`
through the chain those doors call — `replace_faces_offset` /
`replace_face_offset` / `mint_offset` / `T::approx_offset_surface` /
`PropsQuadLane::recertify_approx` / the fit door(s) in
`geom-brep/src/offset_fit.rs` — so that the ONE `tol.eps()` read is at
the site that classifies `sup ‖S_fit − (S + d·n)‖ ≤ ε_precision`, and
no signature on the path carries an `f64` epsilon (grep receipt in the
PR: `tolerance: f64` absent from every signature on the chain).
`SurfaceSpec.tolerance` / `ApproxSurface`'s stored "precision tolerance
the certificate was classified against" is provenance the validator
never reads: keep it as a recorded VALUE if it still says something
true (it now always equals the run's ε — argue whether a field that is
always the global is data or noise; dropping it is acceptable, argued).
Every `FIT_TOL` constant in the tree retires with its parameter; every
existing shell/offset test is green unchanged otherwise (the analytic
offsets never read the value; the NURBS-lane tests in `geom-brep`
classify against ε exactly as before). Red-first where cheap: a fit
forced looser than ε refuses at tier 3 (if not already pinned —
`ApproxCertification`), and the `Tol` witness is the only tolerance
any of these signatures accept (a compile-checked receipt, not a test).

## Acceptance

- The arm: `Verb::Shell` with the record channel, refusals and empty
  flow rows above; the run-door rows; the tag-census amendment as
  closed data; both feature graphs; the costing table per design §6
  against the SEAT-7-amended baseline (a kernel-only verb: rows 1–3 +
  the census amendment; state what a later `Node::Shell` pays).
- The chain: no `f64` epsilon in any signature from the shell doors to
  the classification site; `FIT_TOL` gone; every existing suite green
  with no re-blessing; territory receipt in the PR.
- The NURBS fit's cost at ε ≈ 1e-9: measured once on the existing
  `geom-brep` fit fixture and REPORTED (wall time, refinement rounds,
  reach-or-refuse) — the offset-fit owner's item, not this unit's gate.
- `work/seat/shell-doors-take-tolerance-beside-tol.md` closed in the
  PR with a `## Closed` pointer; lint green.

## Out of scope

`Node::Shell` (LIB-G17); shell's semantics (SHELL-3/4, the curved
wall-clearance gate); changing what the fit engine computes; any
change to `Tol`'s own API beyond passing it.
