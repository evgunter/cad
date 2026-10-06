# Review r1 — PR #4139 "join: a pinch is one vertex per cone, split before the zips"

Frozen head `7863f27b`. Main = `897a2c24` (the PR's merge base) + the PR's one-line `live::linked`
import, so it compiles; release, own target dirs. Lane isolation kept: no other review branch or
session read. Probe: `crates/sweep/examples/r1_4139_probes.rs` (this branch). It reads every
build through `differential::outcome`, tessellates it (δ 0.05, `check_mesh`), and counts the
**cones at each pinch point independently of the kernel**: exact great-circle arrangement of the
oracle planes, cones = in + out − 1, flat cones exempt. Rows and a debug trace: `REVIEW-r1-rows.txt`.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 4 · NOTE 4.
Head ships no wrong body anywhere I looked. It fixes main's crossed bodies at scale. MAJOR-1 is a
definite SOUND→refusal regression that the report says does not exist. It is typed, but it falls
on the operand shape the ruling itself produces. It must be fixed or filed and pinned, and the
report corrected, before merge.

## Findings

**MAJOR-1: a pinched operand whose pre-zip weld fires is split into the wrong cones; 12 SOUND
lines refuse.** Demonstrated by execution.
- Set `dbl`: operand A is two reflex corners touching only at `v` (the kernel's union, two
  vertices on one key, the ruled shape). A cube's near face holds `v`.
- 4320 lines. 16 lines main builds now refuse on head, `ResultInvalid { LoopRoleInverted }`:
  12 main `SOUND` with one vertex at `v` (oracle c1), and 4 main BAD. All are `cube ∖ A`.
- Instrumented head (env-gated `eprintln`): `finish::weld_pinches` fires on exactly 48 lines
  across all my sets, all in `dbl`. Every one refuses on head: the 16 above plus 32 the union
  already refuses on main.
- Trace on `dbl asym asym seed=268 fib11 yx S` (rows file). `split_cones` reads the cube's pierce
  vertex runs `[2,7,10]` as cones `[2,7,2]`, i.e. two cones. It splits the vertex, and two
  vertices remain at `v`. The exact oracle and a brute-force 2880×1440 flood fill both give
  **one** cone. Main's one vertex is the ruled shape.
- So σ_B∘σ_A misgroups here: one B vertex is the weld's product, and the other operand has two
  vertices at the point. `zip.rs:297-309`, `finish.rs:287-300`.
- Falsifies claim 2 ("right in every case") and claim 5 ("the welded vertex is then split per
  cone"). Also falsifies the report's "SOUND → refusal: 3, none definite" beyond its batteries.
- Repro: `R1_PICK="dbl asym asym seed=268 fib11" cargo run -p sweep --release --example
  r1_4139_probes dbl`.
**MINOR-1: the cone composition order is unpinned** (`zip.rs:307`). Executed: mutant
`k = sigma_a[sigma_b[k]]` passes all 23 `join_pierce*` rows. It is byte-identical to head on
stair3, islnotch and dbl except the 16 MAJOR-1 lines, where it refuses `ZipCorrespondence`
instead. No row distinguishes the two orders.
**MINOR-2: `cone_b` is unpinned at three cones too** (`zip.rs:311-314`). Executed: mutant
`cone_b[k] = cone[k]` passes every row and is identical on all 10 800 stair3/islnotch/dbl lines,
including 746 three-cone dbl lines. The PR's "equivalent at two cones" understates it: either the
indirection is redundant, or no reachable pose tests it.
**MINOR-3: open `work/` rows still cite the retired paths** (Q4). The PR's sweep covered
`crates`/`demos`/`tools` only. Found by inspection:
- `work/topo/torn-body-refusal-families-beyond-the-six-doors.md:130` and
  `torn-hops-read-as-absent-across-the-boolean.md:54,90` list `split_across` sites;
- `work/contact/two-copies-of-a-pierce-carry-edges-that-run-within-the-band.md:22` rests on
  `weld_pierce_copies` welding copies a face meets. That premise is gone: copies now stay apart
  more often, so the row's count may move;
- `work/cleave/three-corners-alternating-round-a-corner-refuse-at-the-join.md` is
  `blocked_on` the pinch row this PR builds, and was not re-measured.
**MINOR-4: `split_cones` has no doc comment** (`zip.rs:200-236`, inspection). Its whole block,
`# Errors` included, sits above `type RunsAt`, so rustdoc attaches it to the type alias.
**NOTE-1: curved escalation, as filed.** 123 `cyl` ∩ lines go BAD(operand) → BAD(t3p, operand):
main crossed them (`c2v1`); head builds two solids and the census refuses `CensusUndecidable`. These are the PR's "772 escalated `CensusUndecidable`". 2 cyl lines go BAD → `ResultInvalid`
(`VolumeUncomputable`); 3 go → `VolumeUnmeasured`.
**NOTE-2: the tolerance floor.** In my `nt` set at tilt 1e-7, the kernel's vertex count differs
from the exact cone count on 115 head lines (209 on main). On 39 of them main had one vertex for
one cone and head has two. All are SOUND and mesh. Unsure whether this is a defect or the band's
reading.
**NOTE-3: the self-fusion arm is live:** unreached on head, it fires under "no split" (450 lines).
**NOTE-4: the rows this PR builds stay `open`/`parked`** (presumably closed at merge).

## Claims

1. **Holds, every new body is the ruled shape.** On head, vertex count matches the exact cone
   count on every built line of stair3 (2160), islnotch (4320), multi (10 080), pair (9000),
   x4 (34 560), dbl (4320) and cyl (3600). nt matches except NOTE-2.
   - **Three or more cones.** x4: 248 three-cone lines, `c3v3`. dbl: 1375 three-cone lines.
     Main crossed 746 of them into two vertices; head gives three. x4 and pair go through the
     vertex–vertex lane and are byte-identical to main, mesh panics (pre-existing doubled-edge
     class) included.
   - **Three pinches on one face** (stair3): main crossed 450 lines, head 0.
   - **Island + notch on one face** (islnotch): 112 refusal→SOUND.
   - **Curved, near-tangent:** cyl and r2's nt.
   - Every planar line that newly builds meshes `ok`.
   - **Refusal→BAD re-checked without the census.** r2's 7 near-tangent lines fail t3p only on
     `CensusEscalated`, margin 5.9e-9 in band 1e-9…1e-8; volume, t2 and the certificate pass.
     r2 nt otherwise reproduces the PR exactly: 87→SOUND, 19, 3.
2. **Falsified for pinched operands** (MAJOR-1). Elsewhere the cycles match geometry on every
   built line. Hand derivation: σ_B∘σ_A is right under orbit `h → next(twin h)` with
   antiparallel seams.
   - Re-pairing by edge identity: sound (never refused). Self-fusion: unreached (NOTE-3).
3. **Holds.** `movefac` splits only edge-disconnected pieces, and the pieces sort makes them
   solids: a pinched ∩ gives `SOLIDS 2 SHELLS 2`. The `noshell` mutant refuses `Merge` (180
   stair3, 746 dbl lines). A void touching its outer shell at a pinch was not constructed
   (unsure).
4. **Holds** (sample of 6 lines read). The operand check fails `Containment(VolumeUncertified)`
   in the far-brick union. Volume is at the oracle to 1e-9, and t2, t3′ and the certificate pass:
   right bodies, in main's curved class.
5. **Partly falsified.** No built body carries a crossing weld, because the weld fires on 48
   lines and all refuse. But the welded vertex is not split per cone correctly (MAJOR-1).
6. **Holds.** Byte-identical main vs head: pierce_runs (4536 lines), pinch_runs (3024),
   corner_pairs (16 380), both reflex batteries (1152 each), rc_wide in 12 shards (40 320).
7. **Holds.** PR mutants: "no split" is red on 10 rows (the PR says 9); "no shell split" is red
   on 4. Mine: the σ order and B-cone mutants survive (MINOR-1, MINOR-2).

## Style lane

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6 and Q8 (read `zip.rs` end to end). Q7 below.
- **Q4** — MINOR-3; *sure* for the citations, *likely* that the contact row's count moves.
- **Q5** — MINOR-4; *sure*.
- **Q3** — the pins (`pierce_point_finding`,
  `assert_a_face_runs_through_two_vertices`) never compare vertex count with cone count; a
  crossing is caught only by the mesher's `PinchWedge`. MINOR-1 and MINOR-2 show the cone logic's
  free choices go unpinned. *Likely.*
- **Q6** — the kept weld is a disclosed deviation with a filed row. Its stated premise ("split
  per cone afterwards") is false where it fires (MAJOR-1), so the deviation is not neutral.
  *Sure.*
- **Q7** — a check of the split against link geometry would have caught MAJOR-1; the
  `(runs, cone_of, he_of, sections)` loop with `&dyn Fn` casts reads awkwardly. *Unsure*, taste.
- **Q2** — `zip.rs` module docs still describe `SeamCorrespondence` as "one each, except a
  welded pinch". After a split, `vmap` is rebuilt wholesale from edges, so that sentence now
  only half describes it. *Unsure.*
- **Q1** — no new copy: the prose sweep over zip/ops/finish hits only pre-existing lines, and
  `zip.rs` has no `1e-` literals. Blind to unlabelled logic copies. *Likely.*

REVIEW COMPLETE
