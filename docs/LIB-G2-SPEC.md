# LIB-G2 spec — arc-carrier fillet modes for the PATHS algebra (binding)

Mandate: the last vocabulary gap before the v2 switch (PROFILES-V2
§V7 VQ1(b) sequencing; PATHS-DESIGN §7 banked arc-arrival fillets
"additive, with a use case" — rocker is the recorded use case).
Measured basis: the G2 census (executed 2026-08-08, reproduced in
the PR body's evidence section as needed): **every closed form
already exists, ratified and fuzz-covered, in the S2 sugar**
(`sugar.rs:794-1262` — offset carriers both kinds, signed tangent
points incl. enclosing, arc-length setbacks, major-arc bulge, the
seven gates, the S8 selection ladder). G2 is that machinery given
an algebra door. Deviations numbered and REPORTED; under-specified
interactions are findings-back, not silent fixes.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Every heavy cargo row
`scripts/with-build-slot.sh -- cargo ...`, synchronous FOREGROUND,
long timeouts (≤590000), one at a time; NEVER background or park —
nothing notifies you; stopping kills the run. Run clippy at default
AND `--features interval`, plus the discipline greps, BEFORE
opening the PR (G1's lesson). powi(2) for interval squares of
possibly-zero quantities. Commit AND push per coherent chunk. NO
Co-Authored-By, no model names (blinding). Merge origin/main before
opening; re-merge if main moves; confirm checks STARTED.

## 1. The fence

In scope: `crates/profile` (path.rs, sugar.rs for the extraction,
validate.rs for refusal payloads), `demos/tour/src/rocker.rs`,
`docs/PATHS-DESIGN.md` (§2b addendum). OUT: everything else — no
schema, no editor-core, no sweep, no CI, no renders.

## 2. The extraction (first commit, behavior-preserving — the b1781c2 pattern)

Hoist `fillet_corner`'s candidate loop + selection
(`sugar.rs:534-620` with `Leg`/`ArcCarrier`/`Candidate`/
`offset_line_circle`/`offset_circles`/`nearest_candidate`) into a
`pub(crate)` **`arc_fillet_trims`**-shaped helper mirroring
`line_line_fillet_trims`: `T: Decide` ONLY (no `Bounds` — the
Bounds scope rule; scalar-carried refusal enum; the `.lo()`
diagnostics stay at `fillet_corner`'s door via a mapping, exactly
as `TrimRefusal` does today). `fillet_corner` re-routes through it;
the full existing suite (arc_fillet.rs 27 rows, review_s2,
review_s8_probe) green and bit-identical BEFORE any algebra work
lands — pin with a delegation bit-identity row like
`sugar.rs`'s line×line one.

## 3. The surface design (the new ~35%)

**a. Arc-carrier arrivals — eager resolution is preserved.** The
arrival side of a fillet binds an arc carrier through a new binder
family on the fillet-arrival tip: the arrival anchor is a POINT ON
THE CARRIER plus the carrier itself —
`.at_on(p, center, winding)` (naming is the addendum's call;
`winding` structural as in `arc_center`). The arrival direction is
DERIVED (carrier tangent at `p` × winding), so the bound state is
the existing directed-point lattice slot with the carrier recorded
— the departure side already tracks carriers (`Incoming.carrier`,
`Core.last_arc`); this adds the symmetric arrival bit. §5's
one-struct shape holds (widen the arrival payload, fields private,
binders only). `resolve_fillet` then runs EAGERLY at binding, as
today, with both carriers in hand. The continuation after an
arc-carrier fillet arrival is the arc leg along that carrier to a
target (`to(p)` / `to(Start)` / far-end anchor semantics carry
over; `ArrivalKind` generalizes unchanged per the census).

**b. Corner derivation + the lifted selection rule.** The algebra
forbids authoring the corner, so G2 derives it: incoming ray/
carrier × arrival carrier — line×circle and circle×circle admit
0/1/2 corners. Design (firm in shape): enumerate the JOINT
(corner, fillet-candidate) space — corners filtered by the
advance/reach analogs (in-front-of-incoming as signed sweep on
arcs via `signed_swept`, before-arrival-anchor likewise; the
`path_corner_advance` linear gates generalize to angular ones,
new predicate names through the funnel) — then apply the S8
ladder over the surviving joint set: smallest total setback, ties
→ incoming setback → deterministic enumeration order. This is the
S8 rule lifted one level, with its dominance argument re-stated
for the lifted domain in the addendum. If the dominance argument
genuinely does not lift (a corner-pair where the ladder is
order-dependent), that is a FINDING-BACK with the counterexample —
do not improvise a different rule.

**c. Angular anchors.** The algebra is anchor-pivoted; `Leg` is
corner-pivoted. The extraction's inputs are anchor-fed (the
incoming side's anchor; the arrival anchor `p`), with arc-side
anchor-fit measured as angular margin (the
`FilletLegCarrier::Arc{radius, angular_margin}` diagnostic
already exists — widen `PathError`'s payloads to carry
carrier-kind, retiring the bare `AnchorOutsideTrimmedExtent`
shape where an arc side needs the angular story). Zero-fit and
sign gates follow U2/G1 precedent (funnel-classified, typed).

**d. Refusals retired**: `ArcArrivalFillet` and
`SeamFilletOntoArc` (their property-test pins rework to pin the
new doors). The seam-fillet-onto-arc door makes rocker's mid-arc
outline closure LEGAL under ratified PQ4 (a loop's seam may sit
at a fillet; the seam fillet's arrival carrier is the hub arc) —
state this in the addendum; PQ4's chain rule itself is untouched
and stays pinned.

## 4. Acceptance — rocker migrates, bit-for-bit

- All six rocker fillet sites (5 arc-carrier + the line×line)
  move to the algebra; `close_arc_center` closure stays (G1).
  Differential fixtures per site against the raw `fillet_corner`
  authoring: **bit-identical lowered loops** (the census confirms
  rocker's corners lie exactly on their carriers, so the derived
  corner must land bitwise on the authored one — that derivation
  exactness is THE risk item; if a site cannot be made
  bit-identical, the PR-2 bracket precedent applies: measure,
  report, keep that site raw, and the unit is still acceptable
  with the wall named).
- The eye's arc×arc fillet (the S8 two-survivor corner) is a
  mandatory differential row.
- Byte-identity: tour export trees at all three ε rows vs the
  merge-base, built and diffed yourself in a scratch worktree
  inside the lane. `eye_pick_narration`'s read-back assertions
  unchanged.
- Full profile battery + the three test families extended
  (differential per combination; property rows incl. in-band
  gates for the new angular predicates — G1's NOTE-2 lesson;
  compile-fail rows for the widened arrival surface).
- Zero new [[test]] binaries.

## 5. The §2b addendum

PATHS-DESIGN gains §2b ("G2: arc-carrier fillets") in §2's
register: the arrival binder's consumed DOF, the derived-corner
rule and the lifted selection ladder with its dominance argument,
the angular-anchor semantics, the refusal inventory (retired and
added), and the seam-fillet-onto-arc consequence for PQ4-legal
closures. Fold it into the §3 vocabulary table correctly (G1's
NOTE on table ordering — one Tier-0 block, not an appended
second one).

## 6. PR discipline

One PR preferred; two (extraction, then algebra) if the seam is
cleaner — your call, reported. Report ≤150 lines to
`~/.local/share/cad-work/lib-g2-report.md` with per-phase
token/wall figures. Open, do NOT merge. Final message: PR
number(s) + report path, nothing more. Genuine forks (the
selection-rule dominance and the arrival-binder naming are the
likely spots): report, pick nothing beyond the smallest faithful
reading, flag.
