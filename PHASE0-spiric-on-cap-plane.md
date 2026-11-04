# PHASE 0 — `splice/spiric-on-cap-plane` (notes, removed before the PR)

Item: `work/splice/a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on.md`.
Main at `7072c784`. Probe (uncommitted): an `eprintln!` at
`slit::holding`'s `Placed::Unread` arm, run on
`germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`.

## 1. Measurement, by identity (1e-9 = default, and 1e-6: identical; 1e-12 pending, not completed)

The test passes on main at 1e-9 (2096.6 s) and 1e-6 (486.0 s), with the
β 0.7 union pinned at `"unread rider"`. The probe lines are identical
at both rows: the same keys, carriers, ends and payload.

The 1e-12 run was killed unfinished after more than 2h18m,
single-threaded at 100% CPU. It had not reached the β 0.6 poses. Last
night's nightly 1e-12 full suite did run the test and did not fail it,
and that job finished in 3982 s for the whole suite run in parallel.
So I cannot tell whether the test spins at 1e-12 or this box is slow.
Filed as `work/tcost/the-tilted-rods-other-spins-row-runs-past-two-hours-at-1e-12-locally.md` (P2).
Every unread rider asked in the run is a spiric against the rod's
cylinder wall, `gap CarrierUnread`, `Names { ends: true, record: false }`:

| pose | edge | face | carrier (torus c=0, R 2, r 0.5) | ends |
|---|---|---|---|---|
| β 0.7 (both spins) | `12v1` / `14v1` / `13v1` / `20v1` / `19v1` | `5v5`, `9v5`, `6v9`, `6v3`, `7v3`, `3v1` | Spiric, `axis ∓y`, `u_ref ±(0.6442, 0, 0.7648)` = the rod's axis, `offset ±0.24041` | `(0.98505, ±0.09785, −1.14402)` |
| β 0.6 (both spins) | `12v1` / `18v1` / `19v1` / `28v1` | `5v5`, `3v1`, `6v5`, `6v3` | Spiric, `u_ref ±(0.5646, 0, 0.8253)`, `offset ±0.38364` | `(0.99973, ±0.14953, −1.14879)` |

The face is always the rod's wall: `Cylinder { axis = the spiric's u_ref,
radius 0.15 }` (spin 0's seam along `u_ref (0.7648, 0, −0.6442)`, spin π/2's
along `+y`). At β 0.6 the unread pairs are dismissed by `walls_inside`, so
that pose goes on to `"spiric wall volume"`. At β 0.7 the union refuses
`EdgeInFace { CarrierUnread }` on one of these pairs.

Numerically (β 0.7, edge 12v1): the chord's plane is the cap plane,
with height 0 along the rod axis at every sample. Its open arc runs
inside the rim, as much as 0.0461 m inside at mid-span, and it meets the
wall at its two ends only.

## 2. The plan's premise

- **Planar, plane known structurally?** Yes. A spiric's plane is its
  carrier data, `{(x − center)·u_ref = offset}`, and is not fitted. Here
  `u_ref` is the rod's axis exactly, so `P ⊥` the cylinder axis and
  `P ∩` wall is the rim circle. The reading below does not need `P` to be
  the cap face's plane.
- **`curve_pair::meet` on spiric × circle/ellipse?** The table sends it
  to the **walk** (`method`: `(Spiric, Circle|Ellipse) → Walk`). The walk
  never reads a shared run: `walk::meet` always finishes with
  `shared: None`. So its verdicts are:
  - isolated `points`;
  - `Escalated`/`Touch` (two meetings in one stretch, or a contact in band);
  - `Unsettled::Exhausted` past the piece budget.

  An overlap would come out as a refusal, never as `Off`. That is one-way
  safe, but the walk costs a bisection per rider and can refuse where
  the answer is structural.
- **The walk is not needed: no spiric runs along any conic.** A spiric
  with `offset ≠ 0` is an absolutely irreducible quartic. Its affine
  equation is `(x²+z²+A)² − 4R²x² = 4R²d²`. A real factorisation must be
  into two circle forms `(s + a x + b)`, because the leading form is
  `(x²+z²)²`. Matching coefficients forces `d = 0`. A complex-conjugate
  factorisation has finitely many real points, but the curve has a real
  oval. The mint decides `offset ≠ 0` and the two-oval regime
  (`Curve3::spiric`: `spiric_off_axis`, `spiric_two_ovals`).

  By Bézout it shares at most 8 points with any curve of degree ≤ 2.
  A sphere, cylinder or cone meets `P` in such a curve: none of them
  holds a plane, so `P ∩ S` is a conic, a line pair, a point or empty.
  **A spiric lies along no piece of a sphere, cylinder or cone, with no
  decision needed.** This is the same structural shape as the existing
  arms "no line lies along any piece of a sphere or a ring torus" and "a
  conic out of the plane meets it at two points at most".
- **Off at the shared ends?** `Placed::Off` means off along every piece
  of the span, and contact at isolated points, ends included, is
  allowed. The conic arms already read a conic meeting a surface at its
  ends `Off`. The degree bound caps the contact at 8 points, so the open
  arc reads `Off` whatever happens at the ends.
- **Tangent at an end?** With the degree reading it makes no difference.
  A tangent contact is still one of at most 8 common points, so the
  carrier still reads `Off`. Under `meet` it would cut pieces down to the
  tangency floor and could escalate (two meetings in a stretch), which
  is a refusal. That is one more reason not to route through the walk.
- **A torus wall is different, and the item's claim is wrong there.**
  `P ∩` a torus is a quartic. Only special planes cut a torus in circles
  (a plane ⊥ the axis, a plane through it, a Villarceau plane). The
  quartic can be the spiric itself, for example on the spiric's own
  torus. So a torus can hold the spiric, and the degree argument does
  not reach it.

## 3. Relation to PR 6481 and PR 6437

The proposed reading is a **degree (Bézout) certificate**, not the
polynomial-identity certificate. It samples no point and decides
nothing. It reads `Off` from the carrier kind and the surface kind
alone, just as the `(Line, Sphere | Torus)` arm does. So it is not a
third home for ONECOPY's
`the-spline-polynomial-identity-certificate-has-two-homes`. It is one
more structural arm in the edge home's (`slit::conic_or_line_on`) match.

It returns PR 6437's `Placed::Off`. It never returns `On`, so it stays
one-way.

`spline_off_carrier` (6481) is untouched. Its allow-list (the surfaces
whose residual is the whole real variety) is a different question:
whether a sampled residual certifies off. The degree arm asks nothing
of a residual.

The identity certificate would read a spiric against a torus: one
sample decided off the ring torus's variety reads it off, since the
spiric is irreducible. That would be the third copy, so I propose to
**leave the torus Unread** in this unit. I would file it on ONECOPY's
row as the third reader that row anticipates, adding evidence rather
than opening a new row.

## 4. Proposal

- New arm in `slit::conic_or_line_on`: `(Spiric, Sphere | Cylinder |
  Cone) → Placed::Off`. The comment gives the degree argument in one
  sentence and relies on the mint's `offset ≠ 0`.
- `(Spiric, Torus)` stays `Unread(CarrierUnread)`, and a `Nurbs` carrier
  keeps its arm unchanged.
- One-way: the arm can only say `Off`. Nothing new says `On`.

Witnesses:
- `OTHER_SPINS_STOPS`, β 0.7 union at both spins, re-pinned by identity
  at its next stop (to be measured after the change).
- Kernel rows in `slit.rs`:
  - (a) a spiric on its own torus face reads `Unread`, never `Off`. This
    stands in for "a spiric that runs along the rim conic", which no
    quadric can hold.
  - (b) the β 0.7 chord and its cylinder read `Off`, and a sampled
    differential check confirms it: interior points decided off the
    wall, and only the ends on it.
  - (c) a spiric crossing a cylinder's section conic (its ends off the
    wall and its middle across it) reads `Off`. The brief's negative,
    "must not read Off over a piece where it lies on the wall", cannot
    be built on a quadric. Its realizable form is (a).
- The crease class and feet on another sheet do not arise: the reading
  takes no foot and no derivative.

PHASE 0: waiting for the orchestrator.
