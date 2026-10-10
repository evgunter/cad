---
id: point-in-solid-mints-outside-its-period-and-nappe-rows
kind: unit
title: point_in_solid's wall junction, wall trim, latitude and root-count arms mint INVALID by hand (mints C13-C18)
status: open
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [classification-invariant-family-types-bug-only-states-against-d9-row-4, period-headroom-margin-has-no-shared-home, cone-nappe-is-decided-in-five-places]
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

- **C13 is reachable at small K.** At K = 1.5, a V of two steep pieces
  whose planes each pass 0.9ε from the shared vertex puts a hit 0.9ε round
  the wall at +2.0ε from one plane and −1.6ε from the other. At the default
  K it is unreachable: a definite side is then the side of the vertex's
  height, the same for both planes. Any finite K > 1 is accepted, so this
  is not D9 row 4. It is now
  `PointInSolidError::JunctionSidesDisagree { face, sides, band }`. It
  carries both side margins, is `in_band` (the hit is in the junction's
  band), and is placed by the census as a witness too close.
- **C14 is reachable at K < 2, and it has an answer.** A piece wider than
  Kε and narrower than 2ε has both end junctions within ε of a hit at its
  middle. The arm reads every active junction as just past it, the
  convention the rest of the arm already used. A piece with both ends
  active ends before that point, so it does not cover the hit. The
  parity argument at `wall_outline` covers both crossings. No error
  remains.
- **C15 is not a graze, and it is reachable at the default K.** With
  neither end active, the hit is definitely clear of both ends. Only the
  cosine-window margin's collapse on a narrow piece (ledger F8) puts it
  in band: windows 0.02–0.08 rad wide, with hits 2Kε past or short of the
  end. It now goes through `decide_nonzero` and refuses with the tagged
  decided margin. The cause is filed on INSIDE.
- **C16 is unreachable.** `sphere_chart_trim` returns before the fold on
  an empty level set. Row 0 applies: `latitude_extremes` takes `first`
  and `rest`, so an empty set cannot be passed and the mint is deleted.
  It had no ending to move.
- **C17 is reachable, so it is typed** (the ruling is on
  `classification-invariant-family-types-bug-only-states-against-d9-row-4`).
  It is `PointInSolidError::RootCountDisagrees { face, certified,
  constructed }` and blocks the ray (`confined`). `TorusRoots::CountDisagrees`
  carries both counts.
- **C18 cannot happen.** Row 0 applies: `CircleRoots::CountDisagrees` is
  deleted. The sphere region's `AtApex` arm, which was folded silently
  into a graze, is now an `unreachable!` with why, as is its twin in
  `ops.rs`.

The endings, before → after:

- **C13, through the solid door, `contfp` and the at-rest validator:**
  - Before: the lever plus the unreadable-margin note.
  - After: the lever alone, with both margins in the sentence.
  - The note is gone because both margins were read. Moving the point off
    the junction decides it, which is the in-band sibling's story
    (D4 ¶1 (iv)). The census was `WitnessTooClose(None)` and still is.
- **C14:** the same refusal → an answer (`Some(true)` between floor and
  roof, `Some(false)` outside).
- **C15:** lever plus note → the lever alone, with a decided margin
  inside ε tagged as the gate's rejection. The note is gone because the
  margin was read.
- **C17:**
  - Before: `Fatal`, the whole query refused with the note.
  - After: the ray is blocked and another ray may settle.
  - Where none settles, the sentence names the two counts and the face.
    Its recourse is "test a point farther from that face; this may
    indicate a kernel bug worth reporting". That note is now about two
    counts the kernel computed that disagree, which is true. The old
    note was about an unreadable margin, which was false.
  - The census placement moved from `WitnessTooClose(None)` to
    `WitnessGrazed`, and the validator's from the placement lever to
    the grazed story.
- **C16 and C18:** no reachable ending.

Census over the two files, production code only:

- **Pass 1** (`MarginDiag::INVALID`, `invalid_margin::invalid`,
  `Indeterminate {`, `invalid(`):
  - Before: `solid_contain.rs` had 11 (PRED's `:1289`, `:1922` helper,
    `:1938`, `:1943` and `:2986`, plus C13–C17's `:2699` helper, `:2760`,
    `:2766`, `:2774`, `:3533` and `:5405`). `sphere_region.rs` had 1
    (`:384`).
  - After: `solid_contain.rs` has PRED's 5, and `sphere_region.rs` has 0.
- **Pass 2** (`terminal_sliver:`):
  - Before: `solid_contain.rs` had 5 and `sphere_region.rs` had 0.
  - After: `solid_contain.rs` has PRED's 3, and `sphere_region.rs` has 0.

Pins, run locally at all three ε rows:

- `wall_section_rows`:
  - `a_hit_in_a_steep_junctions_band_carries_both_sides` (4 poses);
  - `a_piece_narrower_than_two_bands_is_read_past_both_ends` (3 widths ×
    3 heights);
  - `a_narrow_windows_collapsed_margin_keeps_its_reading` (3 widths × 2
    sides);
  - `a_hit_between_a_junctions_pieces_carries_both_sides`.
- `torus_predicate_rows`:
  - `the_ladder_disagrees_with_itself_on_a_lanes_coefficients`;
  - `a_count_disagreement_blocks_the_ray_and_says_so`.
