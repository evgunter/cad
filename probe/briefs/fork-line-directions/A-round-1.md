DESIGNER A — round 1 reply (line directions)

## For Ev

**Recommendation as it now stands.** Unchanged in its two halves (a line records director and signed length together; the one-plane reading puts no decided ±1 on a normal). On the form of that reading I now take the other report's *statement* and keep my *place of reading*:

> Two planes are one plane when each contains a point of the other, **that point taken inside the region the verdict is used on**: `n₁·(p₂ − o₁) = 0` and `n₂·(p₁ − o₂) = 0`, where `pᵢ` is the foot of the region's centre on plane `i`.

What moved me: the other report's shape is the better statement. It is two symmetric claims where mine was one lopsided one, each claim reads one plane's normal as a direction only, and it is what the kernel's line-pair reading already does ("each span's ends lie on the other's carrier"). What did not move: where the point is taken. The other report takes each plane's stored origin; I measured that this changes verdicts main gives today. Confidence: **likely**.

**1. The two forms compared.**

- *What each claims.* B: "each plane passes through the other's stored origin". Mine: "the two planes are the same distance from the centre of the region in use". A plane's stored origin is wherever its frame happens to sit: for a cap, the sketch's origin, which can be far from the face. So B's claim is about a point the verdict is not used at; [intrinsic.contingent] is the clause it meets. Written with the feet `pᵢ`, B's first claim *is* my offset (`n₁·(p₂ − o₁) = (n₁·n₂)·d₂ − d₁`) and its second is the mirror image I lacked.
- *In band, measured.* Two planes that cross at the region's centre, tilted `2e-10` (inside the band at arm 1), the second's origin stated `far` along it. Main and my form: one plane at every `far`. B's form as written: one plane at `far = 0`, **undecided at `far = 30`, apart at `far = 1000`**; at tilt `1e-13` and `far = 1e5`, apart. The parallel rung bounds the tilt at the region's arm, and B's offset then levers it at the origin's distance.
- *At each scalar.* Both are zero as expressions whichever way each normal is spelled. B's needs nothing of the normals at all (flip- and scale-free); the foot form needs each normal squared to be 1, which the tier has for a quotient (`S·S = 1`) and which is the unit claim the plane description already makes. At `Interval` both carry a normal's width times a lever; B's lever is the origins' separation, the foot form's is each origin's distance from the centre, as on main.
- *Size of the forms, measured.* B's is linear in one normal and is smaller: under it the "two blocks on one floor" row keeps all its pinned rungs, where the foot form moves two already-unproven rungs' reason to "budget". All seven ring rows pass under either. So B's form costs nothing in the tier and the foot form costs something, on rungs that were unproven anyway.
- *Other readers.* `carrier_eq::reading`'s plane arm has the same `σ·n₂` and takes the same re-posing (its swing becomes `‖n₁ × n₂‖`). `certify.rs`'s `r1 − sigma·r2` is not the same case: each residual is its own zero, no normal is multiplied. The line-pair reading (`curve_pair::pose::one_line`) is already sign-free and already reads at the spans' own ends. `topo::injective::one_plane` compares point values at `Interval` and already admits the negated axis. So one statement serves: *each contains the other's points, at points in use*.
- *If the form size turns out to matter* beyond relabelled rungs: have the caller hand the ladder a point of each plane that lies in the region and is on the plane as a formula (origin plus in-plane coordinates). Then B's linear form is exact and well placed. I would not build that until a lost proof shows it is needed. **unsure** whether it will be.

**2. A direction a test table wrote.** The line-directions work can land either way, and should not wait: **agreed** with B's interim (the table's length is the chord projected on its written direction; honest for a test input). I still think the right final state is that a table carries no directions and the band-accepting check goes, but that belongs to the unit-direction work, where it reopens one point; it is a separate, smaller question for you then, not now. Under B's interim, the table door is the one place a projected length survives, and that should be said where it is written.

**3. The rest.**
- *Verb-authored length:* **agreed**. The rule under both reports is one sentence: the length is whichever spelling makes start + length·director the end with no rewrite. For an angle leg that is the authored length; for a rectangle the authored width and the projected chord are the same polynomial.
- *B's point that main is exposed today* (a rectangle verb's literal wall beside a table polygon's quotient wall): **agreed**, and it is a stronger reason for the reader half than mine: the reader change is owed whatever the doors read.
- *B's README sentence:* **agreed**.
- *The edge's own description* (`SketchSegment::Line { a, b }`): B does not speak to it; my point stands. It is a third statement of the line, from two points, and it is where I expect the remaining 20 decisions of the `sym_9` receipt to be. Until it reads the pair, "every reader reads one statement" is not yet true.
- *A run of collinear sides joined into one:* B does not speak to it; mine stands (first member's director, the lengths summed, as an arc run does).
- *B's "no sign in any director" alternative* and my "teach the tier `|x| = sgn(x)·x`" are the same idea from two ends; we both lean against, for the same reason (the reader fix is needed regardless).

## For the orchestrator

**Your reading of the agreement is right.** One nuance: "no decided ±1 multiplies a normal" is necessary, not sufficient; the form must also read at the consumed extent, which is the remaining difference.

**What I ran** (scratch worktree on main `36cc57d5ea`, removed; nothing pushed). Mutant doors + extent stand-in as in my first report, plus B's form: in `plane_ladder`, `n₂·(o₁ − o₂)` decided first under `plane_eq_offset` (Zero continues, else apart/undecided), then `offset_margin = n₁·(o₂ − o₁)`; in `carrier_eq::reading`, `|n₁·(o₂ − o₁)| + n₂·(o₁ − o₂)` and the cross-product swing.
- `editor-core --test all g_ring_closure_measure two_blocks_resting two_copies_on_one_axis`: ring 7/7 pass; `two_blocks_resting_on_one_floor…` **passes with its pinned rungs** (under my form rungs 4 and 5 read `Budget`); `two_copies_on_one_axis…` red as in every variant including the control (the record's doing).
- `geom-brep --lib plane_eq carrier_eq`: 34/34 pass under B's form.
- A throwaway `geom-brep` test, `plane_ladder` at `f64`, `ConsumedExtent::arm(1.0)`, band `{zero 1e-9, escalate 1e-8}`, `p1 = (origin 0, normal z)`, `p2 = (origin far·(cos τ, 0, −sin τ), normal (sin τ, 0, cos τ))`:
  - B's form: `(2e-10, 0)` One; `(2e-10, 30)` `Undecided`, margin `-6e-9`; `(2e-10, 1e3)` `Apart`, `-2e-7`; `(1e-13, 1e5)` `Apart`, `-1e-8`.
  - `d1 − (n1·n2)·d2`: One, margin 0, at all four. Main's `d1 − σ·d2` gives the same by inspection (both offsets are 0 at the centre); not run.
- Not run: the foot form written as two symmetric claims (I ran only the first of them, which is my original offset); B's form on the `sym_9` receipt; any `Interval` or `1e-12` row.

**How real the far-origin case is.** It needs a genuine tilt inside the band and an origin many arms away. Rounding alone gives tilts near `1e-16`, so at `f64` it takes a near-degenerate pair (a sliver, an imported or solved normal). Over a box it is easier to reach: a normal with width δ over the leaf gives B's offset an enclosure of about δ times the origins' separation, where main's is δ times each origin's distance from the centre. A cap's origin is the sketch origin, so a small face sketched far from its sketch's origin is the ordinary way to get a long lever. I did not measure the `Interval` case.

**On B's sequencing** ("the ladder half can land first, alone, and should not move any existing row"): with B's form that looked true on what I ran (34 unit rows, the floor row). With the foot form two pinned rungs move. Either way the far-origin behaviour should have a row before it lands: the throwaway test above is that row.

**On B's possible pre-existing defect** (rectangle verb beside a table polygon on one wall, over a box): not run by me either; worth the row, it would show the reader half is owed independent of this item.

**Smaller things.**
- B's "`f64`: the axis case cannot be reached" matches the code (`axis_or` needs an expression). B's scalars list names `Sym<f64>`; mine named `Dual`. Both hold.
- B says the `segment_direction` check accepted a table's oblique unit "within the band", so the rim's end is decided numerically there. That is the table door's cost under the interim and is fine for a test input, but `ConstructedProfile::into_profile` also produces such a table from a verb-built profile ("giving up the provenance" while keeping its directions), and that path is not test-only. Under B's interim those sides get a projected length where the verb held an exact one. Either `into_profile` drops the directions (they are re-minted) or it carries the pair; the first is simpler.
- I did not find a fourth plane-identity reading with the `σ·n` shape. I looked at `plane_eq.rs`, `carrier_eq.rs` (`coincident_over_extent_as`/`reading`), `certify.rs` near `gap_orientation`, `curve_pair/pose.rs` `one_line`, `topo/src/injective.rs` `one_plane`/`one_line`. I did not read the stated-axis arm of the coincidence door beyond `one_cylinder`, whose tilt is already `|da × db|`.
