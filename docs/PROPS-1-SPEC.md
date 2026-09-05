# PROPS-1 — the lost-correlation members of the linalg audit: `mirror_across_plane` and `reject_from`

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Linalg
interval honesty; difficulty logged at spec: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting. The
governing contract is `docs/DUAL-DESIGN.md` **DL6**, ratified: this unit
is an audit against it, not a design conversation. The caseload is
`work/props/certified-lane-non-real-contract-audit.md` — the CERT-3
batch of members (the 2026-08-30 comment) and the S-CERT orchestrator's
correction beneath it. Branch `props/1-linalg-lost-correlation`, cut from
`main`.

## What this unit is

Two constructors in `crates/geom-core/src/linalg/` evaluate a correlated
expression naively, so an enclosure that should be tight is paid as width
at `Interval` — nothing about the geometry is ill-posed, a quantity is
subtracted and re-added and the round trip does not cancel in the ring.
The precedent is `Affine3::rotation_about_axis` (`linalg/affine.rs`),
whose translation CERT-3 respelled through
`Mat3::identity_minus_rotation_about` so the vanishing factor is
syntactic. This unit does the same for the two members whose exact
replacement is already written down, and takes the one golden / k-lint
pass both owe together, so the tree re-baselines once rather than twice.

1. **`frame::mirror_across_plane`** (`linalg/frame.rs`). The translation
   is `q − L·q` with `q = point − O`, the anchor mentioned twice; at
   `Interval` it carries `2·width(point)` at every call. The exact
   replacement: a Householder reflection has `I − L = 2·n̂n̂ᵀ`, so the
   translation is `n̂ · (2·(n̂·q))` with `q` mentioned once. The doc
   comment already states this and names the reason it was not done in
   CERT-3 (it moves `f64` bits in the mirror lane). Do it; rewrite the
   doc so it states the present evaluation order only (D9: the order is
   fixed and written down — say which product is taken first and where
   the factor 2 enters) and the invariant, not the history (discipline
   §4).
2. **`Vec3::reject_from`** (`linalg/vec.rs`). `self − self.project_onto(onto)`
   mentions `self` twice, so the rejection carries ~`2·width(self)` and
   does not collapse when `self ∥ onto`. The replacement
   `(onto × self) × onto / |onto|²` mentions `self` once. State honestly
   what it does NOT fix: `onto` is still mentioned three times, as
   before — the gain is on `self`'s width only, and `onto` is usually an
   exact axis. Re-derive the doc's two rounding claims ("orthogonal to
   `onto` up to rounding"; "`project + reject = self` up to one rounding
   per component") for the new spelling — the second is a claim about
   the old association and may no longer hold as written; measure and
   state what does.
3. **`Point2::lerp` / `Point3::lerp`** (`linalg/point.rs`) — **decided
   and LEFT**, the orchestrator's ruling on the audit member: the
   one-difference form's `f64` endpoint behaviour is documented and
   deliberate, and the two-products form buys endpoint symmetry at the
   price of treating `t` and `1 − t` as independent for a wide `t`. Add
   one paragraph to each doc stating the `Interval` cost of the chosen
   form (`2·width(self)` at `t = 1`, exact at `t = 0`) so the decision is
   on the record at the site. No code change.

**Not this unit:** `Mat3::rotation_about`'s `1 − cos` diagonal floor and
the `MappedCurve::restrict` composition growth (the audit's member 5 and
its rider) — filed as `work/props/rotation-about-diagonal-width-floor.md`,
its own decision, because it respells every rotation in the kernel for a
measured sixth of the residue. `Vec3::orthonormal_basis`'s sign hull —
`work/props/interval-orthonormal-basis-sign-hull.md`, the next unit on
the same file. `Vec3::normalize`'s overflow-to-zero — S-CERT's item until
the inheritance. `DUAL-DESIGN.md` itself — M10's file; untouched.

**Fence:** `crates/geom-core/src/linalg/{frame.rs,vec.rs,point.rs}` and
`crates/geom-core/tests/` (one new file, below), PLUS every committed
expectation the `f64` bit movement changes — goldens, k-lint baselines,
render lane rows, python-suite expectations — each named in the PR body
with its consumer path. The consumers are known at dispatch and the lane
traces them first: `mirror_across_plane` ← `pncad-py/src/py/place.rs`
(and `pncad-py/tests/test_placed_union.py`, `ty_fixtures/legal.py`);
`reject_from` ← `topo/src/replace_face.rs:695`, `mesh/src/planar.rs:374`
(a STORED chart frame, `u_ref` — a key-bearing site) and `:1212`,
`editor-core/src/mate/coset.rs` (six sites), `geom-brep/src/offset.rs:192`,
`geom/tests/surfaces/review_m2_pr1.rs`. Anything outside `geom-core`
is another program's ground: edit an expectation there only when this
unit's bit movement is what moved it, say so per site, and re-merge
`main` before every push (`scripts/work.py territory --base origin/main`
lists the crossings; it warns, it does not block).

## Posture

- **Measure first, both respells, before either lands.** An `#[ignore]`d
  evidence instrument `crates/geom-core/tests/props1_evidence.rs` in the
  shape of `cert3_evidence.rs` — corpora as literals, output quoted in
  the PR body — reporting per row: the old and new translation /
  rejection widths at `Interval` for anchors and vectors of stated width
  (metre, 100 m, mm scales; normals exact and wide; the parallel case
  for `reject_from`), and the `f64` difference old-vs-new in ulps. The
  PR body carries the table. If a respell does not narrow on its corpus
  the unit says so and does not land it.
- **Red-first, gating pins** in the same file's non-ignored half (or in
  the module tests): (a) the mirror translation's width at `Interval`
  is no more than the single-mention bound and is narrower than the
  old form on every corpus row; (b) `reject_from`'s width in the
  parallel case is narrower than the old form; (c) containment — the
  new forms ENCLOSE the true reflection / rejection for every point of
  the anchor box on a random corpus, both lanes (the soundness half:
  narrower is worthless if it is wrong). The existing `f64` property
  rows in `frame.rs` (involution, plane fixed pointwise, `det = −1`,
  the poison and refusal rows) stay green unchanged.
- **The `f64` bit movement is the cost, and it is accounted, never
  absorbed** (discipline §3; `memories/output-stability-as-justification.md`).
  Run the consumers' suites; where a committed expectation moves,
  decide whether the new value is right, re-derive it per its own
  runbook (k-lint: `docs/K-REPORT.md`'s recourse — re-derive the
  baseline, never move geometry to get under a threshold; renders: the
  PR REPORTS drift, `main` re-baselines, per `memories/freecad-render-lane.md`),
  and say in the PR body what moved and why. A stored `u_ref` that
  moves by an ulp moves content keys downstream — find out whether it
  does on the corpus and say so.
- **ε posture:** no tolerance decision moves; `definitely_positive` on
  the mirror normal is unchanged. No `CI-Config:` trailer (the run
  gates every lane and ε row; a trailer can only red the classify step).
- **Sweep obligation** (discipline §5): CERT-3's reading sweep found
  these by reading constructors, not by grepping, and named its blind
  spots: a round trip split across a caller/callee boundary, and
  `svd.rs` / `lsq.rs` excluded by a judgement about instantiation. Re-run
  the reading over ALL of `crates/geom-core/src/linalg/` at your merge
  base — every file, `svd.rs` and `lsq.rs` included (say whether they are
  generic over `Real` or concrete `f64`) — for the shape *a derived
  quantity subtracted and re-added, or an operand mentioned twice where
  one mention suffices*. Hit list with disposition, one line each:
  fixed here, filed (report it in the PR body — the orchestrator files
  it; do not write into another program's directory), or not the shape.
  State what reading cannot match.
- **D2-addendum classification:** no variant is retired and no refusal
  is added; say so in one line rather than leaving it implied.
- **Review:** standard v6 dual. Reviewers' first target is SOUNDNESS at
  `Interval` — containment of the new forms over wide inputs, both
  lanes, executed; the second is the drift accounting — every moved
  expectation re-derived rather than adjusted, and every consumer of a
  stored frame checked for a moved key.
- **Landing** (`work/README.md`): the unit item `work/props/PROPS-1.md`
  gets its `pr:` and `status: review` on this branch (state-sync rides
  the unit's PR); the audit item loses members 1 and 3 member by
  member and records member 4 as decided-and-left with a pointer to the
  doc paragraph; `docs/PROPS-1-SPEC.md` is deleted in the landing PR
  per `docs/DOC-LEDGER.md`'s spec lifecycle. No `Co-Authored-By`
  trailer in lane commits; rows spelled out; push early to
  `props/1-linalg-lost-correlation`.

## Acceptance

- Both respells landed with the doc stating the present evaluation
  order, or one respell landed and the other's non-narrowing measured
  and stated; `lerp`'s paragraph in place.
- The evidence instrument committed with its corpora as literals; its
  table in the PR body; the gating pins green in both lanes at every
  ε row on hosted CI.
- Every moved committed expectation named with its consumer, its new
  value argued right, and re-derived by its runbook — none adjusted to
  restore an old number.
- The reading sweep's hit list and its blind spot in the PR body.
