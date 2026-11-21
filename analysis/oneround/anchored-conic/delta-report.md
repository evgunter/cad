# Delta read r1: PR 7203 `oneround/anchored-conic`, `8d5978300..b303a32a6`

Head frozen at `b303a32a6f31badea7aff753db7c35e3fa212f18`. Base
`ec8749552` (unchanged; the delta has no merge). Per
`analysis/oneround/anchored-conic/delta-read-brief.md` at `efbe7687920f`.

## Verdict: APPROVE

Every fix the adjudication asked for is in the tree and does what it
says. Every one of my first-review survivors is now red in the crate
that owns the arm. The local suite at the frozen head reproduces the
implementer's counts, and every red is the base's.

What is left is three NOTEs, none of them gating:

- three wrong paths in the G9 rebuild-site list;
- a dropped row case that nothing replaced;
- a fair answer to G3's question.

One correction to my own first review: its M4 (a third red gate) was
wrong. It came from a stale `origin/main` in my clone. Details under
G10.

**Disclosure.** I did not read the other reviewer's branch. Two
documents I was required to read quote the other review's results:

- the PR body (G3's "review r2's probe: 2`a` off"; G10's "r2's read that
  gate green");
- the new ONEHOME row (its "1 on the head and 33 on the base in the
  other").

I saw those quotations. Nothing below rests on them.

## Executed, and against what

- **`ci` nextest profile, whole workspace, on `b303a32a6`, ε 1e-9:
  17,470 of 17,471.** Red: `pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`.
  Judged against my first-review base run on `ec8749552` (17,459 of
  17,460, same red). It matches the implementer's 17,470 of 17,471,
  which was on `08473553a`.
- **Same at ε 1e-12: 17,468 of 17,471 as run, 17,469 of 17,471 net.**
  - Reds: villarceau, `mesh reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound`,
    and `geom every_suite_file_is_aggregated`.
  - The third was mine: my probe file sat in `geom/tests` during the
    run. With it removed, that row passes on the same build at 1e-12.
  - The other two are the base's 1e-12 reds (base 17,458 of 17,460,
    same two).
- **Seventeen mutants** on the frozen head (`review-r1/delta/mutants2.py`,
  `run2.sh`), each against its owning crate's rows: `geom` (67 rows of
  `conic::` / `curves::tests`), `mesh` (`curve_key|memo::`) or
  `step-import` (`carrier|signature`). **All 17 red.** Table under G1/G4.
- **`scripts/run-gates.sh` with `GATE_BASE=abbff2537`**, on clean
  worktrees of the head and of `ec8749552`. Plus `cover-kind-match.sh`
  alone on both trees at `GATE_BASE` = `b80352cd9` and = `abbff2537` (G10).
- **A probe** (`review-r1/delta/mirror_probe.rs`): an arc-held ellipse
  (a = 1500, b = 15), re-seamed near 0, 1.4, 2.9 and −1.5, mapped by a
  reflection the way `topo/src/transform.rs`'s `map_frame` builds the
  improper frame. Worst distance from the mirror image of the original
  point: 6.8e-13 to 1.2e-12 m at every seam (G3).

Only read:

- `conic.rs` whole (1,774 lines);
- the delta of `curves.rs`, `memo.rs`, `step-import/build.rs`,
  `implicit.rs`, `in_band_sweep.rs`, `intersect.rs` and
  `pcurve_cone_section.rs`;
- the PR body;
- the rows the delta edits or files.

Not run:

- ε 1e-6;
- the slow set, including the 23 cone slow rows;
- clippy, rustdoc and the demo roots.

## The ten items

**G1, confirmed (DEMONSTRATED, sure).** My six first-review survivors,
re-planted on the new code. Each is red in its owning crate:

| mutant | red in |
|---|---|
| `same_content` ignores the form | geom `the_walk_names_an_arc_held_ellipses_anchor_and_phase` |
| `same_content` ignores the phase | the same |
| `data()` omits the phase | the same |
| `map_scalar` drops the phase | geom `a_re_seamed_arc_held_ellipse_reads_back_lifts_and_moves_as_itself` |
| azimuth ignores the phase's sine | the same |
| azimuth ignores the phase's cosine | the same |
| `mapped` drops the phase | the same, and `running_an_ellipse_back_is_its_own_door_and_not_a_frames_reversal` |
| memo key's anchor mark removed | mesh `the_curve_key_parts_a_centre_from_an_anchor` |
| `carrier_sig` drops the phase scalars | step-import `a_carriers_signature_parts_a_centre_from_an_anchor` |
| reversal keeps the sine | geom `a_re_seam_moves_parameter_zero…`, `running_an_ellipse_back…` |
| quarter turn rotates the pair the wrong way | geom `a_re_seam_moves…`, `a_re_seamed_ellipse_resolves…`, `running_an_ellipse_back…` |
| half turn keeps the pair | geom `a_re_seam_moves…` |
| derived centre's sine term sign flipped | geom `a_re_seam_moves…` |
| evaluator reads the derived centre | geom `a_re_seamed_ellipse_resolves…` |

`zero_point` is gone: there is no trait method and no impl
(`grep zero_point` finds nothing in `conic.rs`).

**G2, confirmed (by inspection, sure).** The PR body's width table and
the closed row now name both bases. The `ec8749552` column is the one I
measured to every digit, and "narrower by 5 to 15%" is what those
numbers give.

**G3, a sound resting place (DEMONSTRATED for production's reflection,
by inspection for the rest, likely).**

- `Ellipse::placed` is gone. `mapped` documents that `Frame::reversed`
  is the half turn about the seam line through the stored point, and a
  row pins that beside `reversed`.
- The one production caller of `mapped` is `topo/src/transform.rs:425`
  (`map_carrier`). Under a reflection it hands `mapped` a frame with the
  axis negated. My probe shows that this is the true mirror image at
  every seam, so a phase of `(0, ±1)` survives a mirror. `mapped`'s doc
  says "a rigid map of space"; a reflection with the axial flip is also
  handled correctly. That is a wording narrowness, not a defect.
- What remains one call away (NOTE, likely): `Circle::placed` still
  documents `Frame::reversed` as "the same circle run back"
  (`conic.rs:515-516`), and `Curve3::reversed` spells the two kinds side
  by side, `c.placed(Frame::reversed)` / `e.reversed()`
  (`curves.rs:1559-1560`). A caller copying the circle's spelling onto
  an ellipse gets a defined, documented, wrong-for-that-purpose curve.
  It is no longer silent, only asymmetric. I would not hold the merge
  on it.

**G4, confirmed (DEMONSTRATED, sure).** `the_arc_held_offsets_bound_holds_each_charge_it_names`
goes red in each of these cases:

- the square's own rounding uncharged, with the sine's share kept;
- `sin θ`'s charge dropped in the second offset;
- the half-angle sine's charge dropped.

**G5, true and sufficient by Q6 (by inspection, sure).** The statement
at `crates/geom-brep/tests/pcurve_cone_section.rs:888-895` says:

- what the number is (a tenth over the largest reading, 50 / 81 / 95);
- what it does not hold (the image's construction);
- what it was (48);
- why it moved;
- its register (`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`,
  open), at the claim site.

The ceiling itself is the guard against getting worse. Nothing more is
owed by Q6.

**G6, confirmed; no two conics share a signature (by inspection, sure).**

- `Conic::anchor()` and `Conic::phase()` are gone. Outside `conic.rs`
  the form is read only by datum name, through `CurveData::named` and
  `memo.rs:1098`. I found no other reader by grep.
- `carrier_sig` is `[kind tag] ++ every walked scalar`, compared whole as
  a `Vec<u64>` (`build.rs:287`). Different kinds differ in the tag. Two
  ellipses held differently differ in length (11 against 14 scalars).
  Same kind, same form, same length: every stored scalar is compared.
- So two different conics cannot share a signature. A mutant that drops
  the phase from the signature goes red.
- Signed zeros can still split one curve into two signatures, which is
  harmless here (the PR body says so).

**G7, confirmed (by inspection, sure); one case dropped without a
replacement (NOTE, sure).**

- `Held::{Centre, Arc { phase }}` puts the phase inside the variant,
  and only `conic.rs` matches on it. `Circle` is centre-held. No code,
  test or doc under `crates/` mints or describes a circle held on its
  arc.
- The arc-held arms are still covered: every arc-held mutant above is
  red.
- But `implicit.rs`'s `the_running_residuals_are_the_plain_chain` lost
  its anchored circle, and nothing replaced it (`implicit.rs:3380`
  region; the delta's first hunk). The other two rows that had one use
  `Ellipse::anchored(…, r, r)` in its place. That row still holds one
  arc-held ellipse (1.6, 0.7), so the arm is covered, but the
  equal-semi-axes case is missing there.

**G8, confirmed (by inspection, sure).** I read `conic.rs` whole
against the code:

- One statement of what an anchor is: `:39-49`, "a point of the arc",
  and `:60-61`, "the point at θ = 0 only until the seam is turned".
- The general formula comes first.
- The `f64` re-squaring fact replaces "derived once" (`:66-68`).
- `jet_at`'s "one `sin_cos`" is gone (`:224-226`).
- The `quarter_turned_where` panic now names
  `Ellipse::quarter_turned_where` (`:922`).
- The struct headlines read `center + …`.
- The dead `intersect.rs` paragraph is rewritten to say what the code
  computes.

One stale phrase remains: `running_point`'s doc says "each sine and
cosine read is charged an ulp" (`:262`). The arc-held chain also charges
the square's rounding and the two differences. Trivial.

**G9, mostly right; three paths are wrong (NOTE, DEMONSTRATED, sure).**
The body is corrected ("about a dozen, not the three"). On
`work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md`,
three of the listed files do not exist at those paths:

| listed | actually |
|---|---|
| `crates/topo/src/on_surface.rs` | `crates/geom-brep/src/curve_pair/on_surface.rs` |
| `crates/topo/src/junction.rs` | `crates/sweep/src/blend/junction.rs` |
| `crates/topo/src/end_face.rs` | `crates/sweep/src/blend/open/end_face.rs` |

**G10, settled: two red; my first-review M4 was my error (DEMONSTRATED,
sure).** `run-gates.sh` defaults `GATE_BASE` to `merge-base HEAD
origin/main`. My first review's clone had a stale `origin/main`, so it
compared with `b80352cd9`. After a fetch the merge base of both head and
base with `origin/main` is `abbff2537` (the PR's base sha).

`cover-kind-match` alone:

| tree | `GATE_BASE=b80352cd9` | `GATE_BASE=abbff2537` |
|---|---|---|
| head `b303a32a6` | red (`line_quadric.rs` ball 0 → 2) | OK |
| base `ec8749552` | red (same) | OK |

Whole gate set at `GATE_BASE=abbff2537`: on both trees, exactly
`engine-leaves` (`census.rs` 52 → 53) and `pin-cites-an-open-row`. The
body's account is right.

**Rows new in the delta.**

- `a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`: its
  40 of 40 is now limited to ρ = 1500 and those depths. That is
  consistent with the near-gap row, which builds only ρ = 1500 families.
  The ρ = 1e4 and fuzzed-row numbers it adds rest on the analysis
  branch's probes, which I did not re-run (unsure about the numbers, not
  about the wording).
- `work/onehome/a-stored-frame-reads-as-two-curves-a-rounding-of-the-size-apart.md`
  (P2): its finding is the mechanism I could only guess at in my
  first-review N4. The `Interval` lift encloses the polar factor of the
  stored frame (`enclose_rotation`), not the stored entries.
  - Its "48 of 5,184" for my measurement is right.
  - Its "What it leaves open" and "Not established" sections say no
    more than was measured.
  - I did not re-run its table.

## Style (on the delta)

Questions exercised: Q1, Q3, Q4, Q5, Q6, Q7, and Q8 on `conic.rs`.

- **S1 (Q1, likely).** `Circle::placed` and `Ellipse::mapped` are two
  names for one role with different contracts about `Frame::reversed`
  (G3). That is a near-parallel pair kept by prose
  (`conic.rs:515-516`, `:719-727`).
- **S2 (Q3, sure).** The new rows can fail; every one went red under a
  mutant aimed at it. The memo row's "followed by the phase" assertion
  is an apt guard against the stream-length ambiguity.
- **S3 (Q4, sure).** The three wrong paths on the rebuild list (G9) are
  a list hand-written from two reviews' text rather than from the tree.
- **S4 (Q7, unsure).** `CurveData::named` is the same question
  `Conic::anchor()` was, under a name. That is fine by the adjudication;
  I note only that "nothing outside `conic.rs` asks which point a conic
  holds" (`conic.rs:70-71`) is true of `conic.rs`'s types and not of the
  question.

Ratings on the delta:

- idiom/structure 4 (`Held` with the phase inside its variant removes
  the unread field);
- test quality 4 (17 of 17 mutants red in the owning crate);
- doc/comment honesty 4 (the docs now match the code; one stale phrase
  and three wrong paths).
