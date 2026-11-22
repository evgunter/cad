# Delta read r2 of PR 7203 (`oneround/anchored-conic`), `8d5978300..b303a32a6f31badea7aff753db7c35e3fa212f18`

Frozen head `b303a32a6`. Base `ec8749552`. Local, debug build, 4 cores.

I did not fetch or read the other reviewer's branch. The delta itself quotes
the other review (the PR body and three rows carry figures attributed to
"review r1": 48 of 5,184 enclosures, 1.2 `u·a`, a `cover-kind-match` red on
`line_quadric.rs`, the base's suite counts). I read those where the delta put
them and nowhere else.

**Before the interruption** nothing of this delta read had been run or
written: the first review's files were all that stood on disk.
**Everything below was run after it.**

## Verdict: APPROVE

Every fix I was asked to check is in the tree and does what is claimed. Every survivor of my first review whose arm still exists is red in the crate
that owns the arm. The evaluator is bit for bit the head I reviewed.
The suite's reds are the base's. One MINOR (three wrong paths in a tracker
row), five NOTE; none needs another read.

## Executed, and only read

EXECUTED on `b303a32a6`
- `ci` nextest profile, whole workspace, clean worktree, at ε 1e-9, 1e-12
  and 1e-6.
- 18 mutants (`analysis/oneround/anchored-conic/r2/delta_mutants.py`, log
  beside it), separate worktree and target directory.
- My first review's probe file, unchanged, on the new head: 12,228 lines
  (points, running bounds, lifted and minted `Interval` enclosures, parameter
  read-backs, 107 far-centred cone sections) compared byte for byte with the
  dump taken on `8d5978300`.
- `scripts/run-gates.sh` on a clean head worktree and a clean base worktree;
  `cover-kind-match.sh` alone on the head against five `GATE_BASE`s.
- `sweep`'s `a_ring_on_a_cone_face` and `cone_join_lane` (10 rows, the slow
  ones included) at ε 1e-9 and 1e-12.
- `scripts/work.py lint`.

ONLY READ
- `conic.rs` whole, against its code (G8); the delta of `curves.rs`,
  `memo.rs`, `build.rs`, `intersect.rs`, `implicit.rs`, `exact.rs`,
  `in_band_sweep.rs`, `pcurve_cone_section.rs`, `transform.rs`; the PR body;
  the edited and new rows.
- NOT RUN: clippy, rustdoc, the 23 slow cone rows as a set, the slow set.
  The analysis branch's numbers behind two rows are taken as stated.

## The suite

Judged against the implementer's counts on `08473553a`, and each red against
the base's payload as I took it in the first review.

| run | this head | the implementer's, two commits back |
|---|---|---|
| `ci`, ε 1e-9 | 17,470 of 17,471 | 17,470 |
| `ci`, ε 1e-12 | 17,469 of 17,471 | 17,469 |
| `ci`, ε 1e-6 | 17,470 of 17,471 | 17,470 |

- Red at every ε: `pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`,
  `MapResidual … margin 1.2503179345670444e-9`, the base's message to the
  digit.
- Red at 1e-12 also: `mesh reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound`,
  the base's.
- The certified-scalar ring row (slow set) at 1e-12: `SurfaceResidual`,
  sample 7, in the join, as on the head I reviewed; the other nine cone rows
  pass at 1e-12 and all ten at 1e-9.
- So the two commits after `08473553a` moved no count.

## The ten items

**G1. CONFIRMED.** Re-planted, each judged in the crate that owns the arm
(`geom` whole, 618 rows, unless named):

| first review | re-planted as | now |
|---|---|---|
| R3 / R4 `same_content` ignores the form | D1 (the circle's arm is gone) | RED, `the_walk_names_an_arc_held_ellipses_anchor_and_phase` |
| R5 `same_content` ignores the phase | D2 | RED, the same row |
| R8 the walk omits the phase | D3 | RED in `geom` (that row) and in `step-import` (`a_carriers_signature_parts_…`); green in `mesh`, as its row's doc says |
| R6 `map_scalar` drops the phase | D4 | RED, `a_re_seamed_arc_held_ellipse_reads_back_lifts_and_moves_as_itself` |
| R9 / R10 azimuth ignores the phase's sine / cosine | D5 / D6 | RED, the same row |
| R7 the placement map drops the phase | D7 | RED, that row and `running_an_ellipse_back_…` |
| A6 the memo key's mark removed | D8 | RED, `mesh` |
| A5 the square's rounding uncharged | D9 | RED, `the_arc_held_offsets_bound_holds_each_charge_it_names` |
| R16 `sin θ`'s charge dropped | D10 | RED, the same row |
| R14 the import signature's mark | no mark now; D16, the signature reading fewer scalars | RED, `step-import` |
| R17 the anchored circle's lift | the arm is removed | — |
| R18 `zero_point` | the method is removed (no `fn zero_point` and no caller in `crates`) | — |

Others of mine, to see nothing fell out: the walk naming an anchor `Center`
(D11: RED in `geom` and `mesh`, green in `step-import`, where a signature
with no mark cannot see a name); a reversal keeping the sine (D12, RED); the
quarter and half turn's phase dropped (D13, D14, RED); the evaluator reading
the derived centre (D15, RED); the difference's charge dropped (D17, RED).
D18 is NOTE d1.

**G2. CONFIRMED.** The PR body and the closed row name both bases and say 5
to 15%. The `ec8749552` column is the one I measured.

**G3. CONFIRMED, and I think it is a sound resting place** (likely).
`placed` is gone from `Ellipse`; `mapped`, `translated`, `reversed` are three
doors; `Curve3::reversed` and `Curve3::translated` take the named ones. What
`mapped(Frame::reversed)` draws is what its doc says: I checked the algebra
(the half turn about the line through the stored point along `u_ref` sends
`A + x·u + y·v` to `A + x·u − y·v`, which is the reversed frame's point at
the same parameter), and the row pins it against that half turn, against
`reversed`, and at `2·semi_v` apart. A wrong curve is still one call away
for a caller who means "run it back" and reaches for the frame's reversal.
What is left of the trap is NOTE d2.

**G4. CONFIRMED.** D9, D10 and D17 each go red. The row now states the
bound's three terms and asserts each, so it is a pin on the charges the code
makes, with `2·rounding ≤ squares` beside it as the one line that says a
charge is enough.

**G5. TRUE AND SUFFICIENT by Q6** (sure). The constant's doc
(`pcurve_cone_section.rs:883`–`:896`, the constant at `:896`) says it is a measured ceiling, a tenth
over 95 (104 against 104.5), not a pin, that it was 48, and names the open
pcert row as its register; the row's doc says its mutant reds at one pose of
three and reads under the unmoved figure at two. That is a measurement with a
scheduled register and a reason at the claim site.

**G6. CONFIRMED; two different conics cannot share a signature** (sure).
`Conic::anchor()` and `Conic::phase()` are gone. Outside `conic.rs` the only
production reader of the form is `mesh/src/memo.rs:1098`, by datum name;
the oracle and two test rows read `data().named(…)`. A signature is a
`Vec<u64>` compared whole: a centre-held ellipse's is 12 words and an
arc-held one's 15, the kind's tag leads, and every stored scalar is in it, so
two ellipses with one signature store the same placement, the same sizes and
the same phase. The row asserts the two apart and the length's difference of
three; D16 reds it. The mesh key keeps its mark because it is a stream, and
its row now shows the mark parting what the length would not (D8).

**G7. CONFIRMED.** `Held::Centre | Held::Arc { phase }`, matched in
`conic.rs` only; no unread field. No `Circle::anchored`, and nothing in
`crates`, `work` or `docs` documents or tests a circle held on its arc
(`rg`). Coverage of the ellipse's arc-held arms is not less: the two rows
that used an anchored circle use ellipses of equal semi-axes
(`implicit.rs`, `conic.rs:1242`), and the evaluator, the bound and the
enclosures are bit for bit the earlier head's on 107 sections. Residue:
NOTE d3.

**G8. CONFIRMED** (sure). Read whole against the code. One statement of the
anchor ("a point of the ellipse"; "the point at `θ = 0` only until the seam
is turned"); the general formula first; the `f64` re-squaring said in place
of "derived once"; `jet_at`; `quarter_turned_where`'s `# Panics` and the
door name it passes; the seam convention; the dead comment in the mint. I
found no sentence the code contradicts. One word: NOTE d4.

**G9. CONFIRMED in the PR body; the row's list has three wrong paths.** m1.

**G10. SETTLED: two gates are red, and `cover-kind-match` turns on
`GATE_BASE`** (sure). `scripts/run-gates.sh`, `GATE_BASE` unset, which the
script resolves to `ec8749552` (the merge base with `origin/main` at
`fd7e5f47`, which now holds the base): head and base each red on
`engine-leaves` (`census.rs` 52 → 53) and `pin-cites-an-open-row`, the same
lines, and nothing else. In my first review the script resolved it to
`abbff253` and read the same two. `cover-kind-match.sh` on the head:

| `GATE_BASE` | result |
|---|---|
| `ec8749552` (today's merge base) | OK, 411 sites |
| `abbff253` (the merge base at my first review) | OK |
| `3df3b1d41` | OK |
| `b80352cd9` (an older merge base, the one the rows name for phase 1) | RED: `ball crates/geom-brep/src/curve_face/line_quadric.rs: 0 -> 2` |
| `fd7e5f47`, `699a1d2a` (tips of `main`, not ancestors) | RED: `arm implicit.rs 2 -> 7` (and `join.rs 15 -> 17` at the first) |

So a red on `line_quadric.rs` is what a stale `origin/main` gives. Against
its merge base the gate is green on head and base alike. The reds against a
tip of `main` say the counts must fall when this branch takes `main`; they
are the base's counts too.

## Also new in the delta

- `a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`: the added
  paragraph narrows the 40 of 40 to ρ = 1500 and those depths. It contradicts
  nothing I measured (I printed 40 of 40 at exactly that pose set); its own
  figures are the analysis branch's, which I did not run. Unsure beyond that.
- `work/onehome/a-stored-frame-reads-as-two-curves-…` (P2): what it cites of
  my review is right (1 enclosure on the head, 33 on the base, at a quarter
  or half turn) and its reading of it is the one I gave as likely. Its table
  of root separations is the analysis branch's; not checked.
- The far-arc cover's row and the PR body's new section: the same order as my
  cover table, taken through the ball where I read `range_along`; "neither
  adds to it nor removes it" is what I found.
- The unit's row carries my two notes as written.

## Findings

**m1. MINOR. Three paths in the rebuild list do not exist.** DEMONSTRATED
(`ls`), sure. `work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md`,
"The doors that rebuild a conic through a centre constructor":
`crates/topo/src/on_surface.rs` is `crates/geom-brep/src/curve_pair/on_surface.rs`;
`crates/topo/src/junction.rs` is `crates/sweep/src/blend/junction.rs`;
`crates/topo/src/end_face.rs` is `crates/sweep/src/blend/open/end_face.rs`.
My first report gave the three as bare file names, which is where the guess
came from. `work.py lint` passes; it does not check paths.

**d1. NOTE.** `Ellipse::centred_at`'s arc arm has no row in `geom`: setting
the origin there (D18) passes all 618 and is red in seven `geom-brep` rows.
It was so at the first review (R12) and was not one of the ten. Sure.

**d2. NOTE.** `Circle::placed` still documents `Frame::reversed` as the way
to run a circle back (`conic.rs:514`–`:516`), and `Curve3::reversed` uses
it, one line above `e.reversed()`. So the one idiom is right on a circle and
another curve on an ellipse held off its seam's vertex. Stated and pinned,
not removed. Likely; style.

**d3. NOTE.** `in_band_sweep.rs:753` keeps `(None, Born::Anchored) =>
unreachable!(…)`: the test's own `Born` enum can still say "an anchored
circle", and the table filters it out with a `None`. `carrier_resolution.rs:414`
still says "born with its centre". Sure; style.

**d4. NOTE.** `Ellipse::mapped` says "a rigid map of space", and
`topo::transform::map_carrier` hands it a reflection's frame map as well
(`transform.rs:425`). The property the doc states holds there. Unsure it is
worth a word.

**d5. NOTE.** D3 and D11 together (the walk omitting the phase AND naming the
anchor `Center`) would give an arc-held ellipse a centre-held one's
signature; each alone is red in `geom`. Not a defect. Sure.

## Style

Exercised on the delta: Q1 (the two `stored` bodies and the two `centred_at`
are one each now the circle's are gone; `offsets` / `running_offsets` still
two, still held by a bit-equality row), Q3 (by mutation, above), Q4 (m1 is a
sentence that cites what is not there), Q5 and Q8 (G8), Q6 (G5), Q7 (G7 is how
I would have held it). The fix pass minted no new instance of what it closed
that I could find: the new rows each go red on the arm their sentence names.

Ratings on the delta: idiom / structure 5 (the phase lives in the variant
that reads it); test quality 4 (every arm of the form's identity and of the
re-seamed ellipse is held in its own crate; `centred_at`'s arc arm is not);
doc / comment honesty 4 (the module, the PR body and the rows say what the
code does, less three paths).
