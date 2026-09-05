---
id: interval-orthonormal-basis-sign-hull
kind: issue
title: Vec3::orthonormal_basis returns a sign-hulled frame at Interval when n.z encloses zero
status: open
opened: 2026-09-03
refs: [1191, 1939]
needs_ev: true
pr: 1939
---

## What was measured

`crates/geom-core/src/linalg/vec.rs:405` (`Vec3::orthonormal_basis`)
opens with

```rust
let s = T::one().copysign(self.z);
```

At `T = Interval`, `Real::copysign`'s zero-containing arm applies
whenever `self.z` encloses zero: it returns the two-sided hull
`[-|x|, |x|]` with the decoration capped at `Def`
(`crates/geom-core/src/interval.rs`, `copysign`'s own docs, which state
this convention deliberately). So `s = [-1, 1]`, and both returned
basis vectors carry `s` as a factor.

Every planar face whose normal has `n.z == 0` therefore stores a
sign-hulled `u_ref`. That is EVERY vertical wall of an extruded prism:
`newell_plane` (`crates/geom-brep/src/newell.rs:157`) takes the frame
straight from `orthonormal_basis`, and an extrude's side planes are
built through it.

Observed on a literal 12-gon prism replayed at `Interval` over an
ε-scaled parameter box (M10-5's fixtures): one wall's stored frame came
back as

```
u_ref: Vec3 { x: [-6.25e-11, 6.25e-11], y: [-0.0, -0.0],
              z: [-1.0000000000312517, 1.0000000000312517] (Def) }
```

for a face whose true chart direction is exactly `+z`.

## Why it matters

The value channel is unaffected — the plane's LOCUS is still enclosed —
so nothing that only evaluates points is wrong. What breaks is any
consumer that REFINES the chart: `Surface::eval` over a `(u, v)`
sub-rectangle of such a plane returns the whole face's box however far
the rectangle is narrowed, because the frame vector itself spans both
signs. The measured cost (PR #1939, table 4) is a 2× enclosure — the mirrored
face is hulled in — not a stalled refinement: `refines` is true at both
a metre-scale and an ε-scaled window today.

M10-5's clearance engine met this directly. It works around it by
re-charting planar carriers at its own door
(`editor_core::clearance::in_plane_axis` / `chart_frame`: the normal
crossed with a world axis chosen by widest cross product, normalized —
no `copysign` anywhere on that path), and by refusing typed when a
chart provably does not refine. The workaround is local to that engine;
every other chart consumer still reads the stored frame.

## What a fix would look like

The basis is Duff's branchless construction, and the branch it avoids
is exactly the one interval arithmetic cannot take. Two shapes are
plausible, and neither is this unit's to choose:

- pick the crossing axis from the normal's own components (largest
  |component| under a total order) and normalize the cross product,
  which needs no sign transfer at all;
- keep Duff's form but supply `s` from a decided sign rather than
  `copysign`, refusing typed when the normal's z-component is not
  sign-definite.

The first changes stored frames on existing documents and therefore
moves content keys; the second turns a total function into a partial
one. Both are geom-core decisions with a wide blast radius.

## Sized (PROPS orchestrator, 2026-09-05 — a read-only census)

Corrections to the premise above, at `main` today:

- **The #1157 mechanism is already fixed** (`vec.rs:405-412`): the
  denominator is `1 + |n.z|`, so nothing is unbounded and `b1` carries
  `s` only in `b1.z = −(s·n.x)`. For a wall with `n.x = 0` the stored
  frame is exact; for a wall with `|n.x|` near 1 it is still the whole
  `[−1, 1]` hull the measurement above shows. The item stands for
  every wall not aligned with the y-axis.
- **The hull is pinned as a decision**: `vec.rs`'s
  `orthonormal_basis_at_a_vertical_plane_is_bounded_and_certified`
  says a spelling that narrowed it by deciding the sign would red
  there, and `Interval::copysign`'s doc (`interval.rs:347-360`) argues
  the two-sided hull is what keeps an f64 replay seeing `−0.0`
  contained. Changing it is a design fork, not an audit fix.
- **Blast radius of changing the f64 frame** (option 1 above): content
  keys never reach `u_ref` (`ContentBits` is a scalar trait; no
  surface feed); but `Datum::FaceFrame` stores a spin *relative to the
  carrier's stored `u_ref`*, so a changed constructor rotates every
  saved sketch on a wall with the file bytes unchanged; STEP export
  writes `u_ref` verbatim and 18 fixtures are byte-golden; ~19 tests
  pin frame bits. Option 1 is rejected on the silent-rotation cost.
- **Option 2** (refuse typed when `n.z` is not sign-definite) refuses
  every vertical wall at `Interval`. Rejected.
- **The shape that remains**: an `Interval`-only narrowing with no f64
  bit moved. Whether a POINT enclosure at zero carries its sign bit
  through the backend (`from_f64(−0.0)` and the Newell cross-sum at
  `Interval`) is a measurable fact that decides between (c) transfer
  the bit at a point zero — zero f64 change — and (c′) canonicalise
  the zero at f64 (`copysign(1, n.z + 0)`), which moves bits only on
  walls whose Newell normal has `z = −0.0` today. Both need a corpus
  census (how many walls carry `−0.0`; which STEP `u_ref` records
  move under (c′)). That measurement is the next lane on this file;
  its numbers go to Ev as an `[ev]` ruling with (c)/(c′) argued.
  M10-5's `in_plane_axis`/`chart_frame` workaround retires after.

## Question for Ev (PROPS orchestrator, 2026-09-05) — which Interval-only fix

The four measurements are merged as PR #1939 (instruments committed,
tables in its body). What they decide:

1. **The backend does not carry a signed zero through `*` and `/`.**
   `interval.rs` wraps the in-repo `interval-transcendentals`, not
   inari (a stale comment says otherwise). `from_f64(±0.0)`, `+`, `−`
   and unary `−` keep the bit; `(−1)·[+0,+0]` gives `[+0,+0]` where
   f64 gives `−0.0`, and `normalize((1,0,−0)).z` likewise — and
   `normalize` is on every `newell_plane` path. So option **(c)** —
   transfer the sign bit at a point-zero enclosure — is NOT sound as
   the tree stands: the Interval replay sees `+0` where the f64
   program saw `−0` and the enclosure would exclude the f64 frame.
   Making it sound means a backend invariant (signed zeros preserved
   by every op on every path), which is a heavier discipline than the
   hull it would replace.
2. **Census, 815 planar faces over three corpora**: exactly 12 walls
   carry `n.z = −0.0`, all in `die` and `kiss_assembly`, all minted
   by the boolean's face reversal (an extruded cube has four `+0.0`
   walls; `cube − cutter` has four `−0.0` walls); a Newell sum itself
   never mints `−0.0`. Zero in the wild corpus and the Band 4 corpus;
   zero faces in the `|z| < 1e-12` nonzero class anywhere.
3. **Under (c′)** — canonicalise at f64, `s = copysign(1, n.z + 0)` —
   all 12 move: 8 `DIRECTION` records in two byte-golden STEP fixtures
   (`die.step`, `kiss_assembly.step`); four of the twelve flip `u_ref`
   by a half-turn. No `FaceFrame` sits on any of them; the committed
   `.pncad` documents carry no `FaceFrame` at all.
4. **Payoff on M10-5's 12-gon prism**: 6 of 12 walls narrow `u_ref.z`
   from width 2 to ≤ 7.4e-15 and halve the cell's z-enclosure; 4 were
   already exact (`n.x = 0`); **2 stay hulled** because their `n.z` is
   `[−2.2e-16, 2.2e-16]` at Interval though exactly `0.0` at f64 — an
   honestly wide input, not this item's defect.

**Recommendation: (c′)**, inside `orthonormal_basis` so every producer
is covered: `copysign(1, n.z + 0)` at f64 makes the frame independent
of the zero's sign, and then an Interval point-zero arm answering `+`
encloses the f64 program by construction, whatever the backend does
with sign bits. Cost, stated plainly: the 12 corpus walls' stored
`u_ref` moves (4 flip), the 8 STEP records re-derive, and **any user
document with a `FaceFrame` on a boolean-reversed vertical wall would
rotate by a half-turn at its next evaluation** — none in any corpus,
but the document format has no migration channel for it, so this is
the one thing to accept or refuse. (c) is rejected on point 1; option
1 (a different frame construction) was rejected in §Sized on the same
silent-rotation cost at every wall rather than twelve.

If (c′): the unit lands the respell, the point-zero arm, the 8
re-derived records with the reason, a doc line at `Datum::FaceFrame`
naming the class, and closes this item; M10-5's `chart_frame`
workaround retires for the point-zero class only (the two
`[−2.2e-16, 2.2e-16]` walls keep needing it). If refused: the item
records the hull as a decision and the workaround stays.

## Home

`crates/geom-core/src/linalg/vec.rs` (`orthonormal_basis`), consumed by
`crates/geom-brep/src/newell.rs`. Related to issue 1191 only by
symptom: that one is about enclosure WIDTH growing with the parameter
box, this one is about a frame that is degenerate at any width.

## Rider — the same body also names `s` twice (PROPS-1's sweep, 2026-09-05)

Independent of the `copysign` hull, `orthonormal_basis`'s second basis
vector is spelled

```rust
let b2 = Self::new(-(s * br), s - s * (self.y.powi(2) * r), -self.y);
```

`s` is named twice in the `y` component where `s * (T::one() - self.y.powi(2) * r)`
names it once (`crates/geom-core/src/linalg/vec.rs:449`). At `Interval`
that is this program's lost-correlation shape: a two-sided `s` is hulled
twice and the entry widens to `[-1-δ, 1+δ]` where scaling once would give
`[-(1-y²r), 1-y²r]`. It compounds with the hull rather than replacing the
finding above — retiring the double mention narrows the entry but does
not make a sign-hulled frame usable — so both belong to whoever takes
this item, and the respell should ride the same golden pass as the hull
fix rather than a second one.

**The site already argues against the single-mention spelling, and the
argument has to be answered rather than ignored.** `vec.rs:409-413` says
both `s` mentions are kept for `f64` bit identity; the r2 review lane
measured what a respell costs there — `s * (1 - y²r)` is bit-identical
to `s - s*(y²r)` everywhere except a signed zero at `n = (0, ±1, −0.0)`,
where the shipped spelling gives one sign and the respell the other. So
the respell is not free at `f64`, it is one flipped signed zero, and
whoever takes this item owes that ledger entry (and a check of whether
any stored frame's content key reads that bit) alongside the `Interval`
gain.

Found by PROPS-1's reading sweep over `crates/geom-core/src/linalg/`;
not fixed there because this item owns the site.
