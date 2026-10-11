---
id: point-in-solid-mints-outside-its-period-and-nappe-rows
kind: unit
title: point_in_solid's wall junction, wall trim, latitude and root-count arms mint INVALID by hand (mints C13-C18)
status: closed
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [classification-invariant-family-types-bug-only-states-against-d9-row-4, period-headroom-margin-has-no-shared-home, cone-nappe-is-decided-in-five-places]
branch: cleave/point-in-solid-mints
closed: 2026-10-11
pr: 4607
---

Unit 3 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`).

## Sites

All are in `boolean/solid_contain.rs` unless named otherwise.

- **C13, `:2760` (`bool_wall_junction`, the pieces' sides disagree).**
  C14, `:2766` (both ends of a piece are active).
  - Two decided verdicts that cannot both hold.
  - Each is either an invariant (a panic, D9 row 4) or a typed finding
    carrying both readings. Read each and decide (design item 7).
- **C15, `:2774` (`bool_wall_trim` decided Zero).** Its sibling Zero
  arms answer `Ok(None)`, a graze.
  - Decide whether this one is a graze too.
  - If it is not, route it through `decide_nonzero`.
- **C16, `:3534` (`latitude_extremes`, no levels).** A structural empty,
  with no margin read. It should be a typed `PointInSolidError` arm, not
  an `Indeterminate`.
- **C17, `:5405` (`bool_ray_torus_count`).** C18,
  `boolean/sphere_region.rs:384` (`bool_sphere_region_roots_count`).
  - `CountDisagrees` is a broken invariant.
  - Whether it panics or stays typed is
    `classification-invariant-family-types-bug-only-states-against-d9-row-4`'s
    call. Take that row's answer, or settle it here and say so there.

## Not this unit

- PRED's four sites stay on PRED's rows: `:1289` and `:2987`
  (`bool_wall_trim_period`) on `period-headroom-margin-has-no-shared-home`,
  and `:1938`/`:1943` (`cone_nappe`) on `cone-nappe-is-decided-in-five-places`.
- PR 4497 accepted `PointInSolidError::Escalated`'s ending (the
  placement lever plus the note). Moving an arm off `INVALID` changes
  which note it takes, so pin it.

## Built (branch `cleave/point-in-solid-mints`)

Measured on `origin/main` `1e816fc9a3` first, at ε = 1e-9, 1e-6 and 1e-12.
Every site below minted `MarginDiag::INVALID`. Through `point_in_solid`,
the containment door (`contfp`) and a ray, each ended "margin is invalid
(NaN or a refused enclosure) ... Recourse: move the point clearly inside
or outside the face; an unreadable margin may indicate a kernel bug worth
reporting".

- **C13 is reachable at small K.**
  - Witness at K = 1.5: two steep pieces whose planes each pass 0.9ε from
    the shared vertex form a V. A hit 0.9ε round the wall reads +2.0ε
    from one plane and −1.6ε from the other.
  - The review put the threshold at K* = 2, up to about 3.2 with radial
    misses. At the default K it is unreachable.
  - Now `PointInSolidError::JunctionSidesDisagree { face, sides, band }`,
    `in_band`, placed by the census as a witness too close.
- **C14 is reachable at K < 2, and it has an answer.** The arm reads every
  active junction as just past it. A piece with both ends active ends
  before that point, so it does not cover the hit. The review checked
  this on 75,108 oracle samples with 0 wrong. The pin is a family checked
  against the parity at the hit's exact azimuth: flat floors, tilted
  ones, a tilted step, a flat step up, and a face that is itself the
  sliver and turns at both ends.
- **C15 is not a graze, and it is reachable at the default K.** With
  neither end active, the hit is definitely clear of both ends. Only the
  cosine-window margin's collapse on a narrow piece (ledger F8) puts it in
  band, for windows 0.02–0.08 rad wide.
  - Now `PointInSolidError::NarrowWindowUnread { face, diag }`, carrying
    `decide_nonzero`'s tagged rejection. It names the narrow window and
    ends on moving the point plus the kernel-limit note.
  - The cause is filed on INSIDE.
- **C16 is reachable.** A face closed on itself along one slit arc (a
  latitude arc or an equator arc) has no rim and no vertex on a pole, so
  its level set is empty. Before, it answered "a face the chart cannot
  express", so `boxes` took the whole ball. It now answers the
  unconstrained trim, the whole sphere, which is the same box.
  `latitude_extremes` takes a non-empty set by type.
- **C17 is reachable, so it is typed.** The cause is a kernel defect:
  rung 1 certifies on `Δ`'s uncharged `f64` rounding, so the certified
  count is the wrong one, and the construction is right. The review
  counted the pinned quartics exactly, and a ray at `R = 1e6` reaches it.
  - Now `PointInSolidError::RootCountDisagrees`. It blocks that ray
    through `torus_crossings`, the helper `cast_ray` calls.
  - The defect also gives silent wrong `Miss` answers. P1 filed on INSIDE.
  - The ruling is on
    `classification-invariant-family-types-bug-only-states-against-d9-row-4`.
- **C18 cannot happen.** `CircleRoots::CountDisagrees` is deleted. The
  first-harmonic doors (`first_harmonic_roots`, `constant_residual_roots`)
  answer `FirstHarmonicRoots`, which has no apex and no count
  disagreement. The two `unreachable!` `AtApex` arms in `sphere_region`
  and `ops.rs` are gone with it.

Endings, before → after (the C13 and C15 states, and C17's
three doors, are pinned through the solid door, `contfp`, the at-rest
validator and the census):

- **C13:** the lever plus the unreadable-margin note → the lever alone,
  through `unnamed_placement_ending` (the `None` arm of
  `placement_ending`), with both margins in the sentence. The note is gone
  because both margins were read: this is the in-band sibling's story
  (D4 ¶1 (iv)).
- **C14:** the same refusal → an answer.
- **C15:** the lever plus the unreadable-margin note → "test a point
  farther from that end", plus "a limit of the kernel's narrow-window
  reading, worth reporting". The census places
  it as `WitnessNarrowWindow`, and the validator gives the same story.
- **C17:**
  - Before: the whole query was refused, with the unreadable-margin note.
  - After: the ray is blocked. Where no ray settles, every door gives its
    lever plus "this may indicate a kernel bug worth reporting". The
    census places it as `WitnessRootCount`.
- **C16:** the ball → the same box, now reached as the whole-sphere trim.
- **C18:** no reachable ending.

Census over the two files, production code only. The second pass also
counts the impossible variants of a shared result type that are read as
outcomes:

| pass | file | before | after |
|---|---|---|---|
| 1 (`INVALID`, `invalid_margin::invalid`, `Indeterminate {`, `invalid(`) | `solid_contain.rs` | 11 | 5 (PRED's) |
| 1 | `sphere_region.rs` | 1 | 0 |
| 2 (`terminal_sliver:`) | `solid_contain.rs` | 5 | 3 (PRED's) |
| 2 | `sphere_region.rs` | 0 | 0 |
| impossible variants read as outcomes | `sphere_region.rs` | `AtApex` | 0 |
| impossible variants read as outcomes | `ops.rs` | `AtApex` | 0 |

## Closed (PR 4607, 2026-10-11)

C13–C18 are off `INVALID`:
- **C13:** `JunctionSidesDisagree`, reachable only at K < 2.
- **C14:** answers. A piece with both ends active does not cover the hit, checked against
  exact parity.
- **C15:** `NarrowWindowUnread`, with its cause on INSIDE.
- **C16:** the whole sphere answers its unconstrained trim.
- **C17:** `RootCountDisagrees` blocks the ray, and its story at every door is the lever plus the
  kernel-bug note. The certified count is the wrong one (rung 1's uncharged Δ rounding), filed P1 on
  INSIDE: `ray-torus-count-certifies-on-an-uncharged-discriminant`.
- **C18:** deleted, as row 0.

The first-harmonic doors return `FirstHarmonicRoots`, so `AtApex` cannot be represented there.
