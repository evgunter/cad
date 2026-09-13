# PERF log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/perf/plan.md`. A/B band 3400–3499
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening for work (2026-09-10)

PERF has been a register since 2026-07-21 (no orchestrator, no units).
Ev opened it for work in-chat on 2026-09-10 with these rulings, which
this entry is the record of:

- **Territory.** Few other orchestrators are running, so PERF lands
  fixes in whichever program's territory holds the cost and records the
  seam here; the owner's dispatched/review rows are checked first, and
  very new code is deprioritized (it will change anyway, and a perf
  change on it is work done twice).
- **Developer-side pain is in scope** (debug profile, CI, test wall
  time), not only the GUI and kernel-API seats.
- **"Stop doing this" beats "do this faster"** whenever both are
  viable — but a change of that kind that removes work a ratified
  clause asks for (D1's per-op tier-1 postcondition is the standing
  example) is an `[ev]` PR before it lands, not after.
- **Review posture: full v6 dual** on every kernel unit unless the
  orchestrator judges a unit unusually low-risk and says so in its row;
  measurement-only and instrument-only units record no row.
- **Merging.** The orchestrator merges its own units after the dual
  concludes (Ev, in-chat).
- **Ev's own pain point:** the GUI lags after edits, location unknown.
  That is the first thing the exploration wave measures.
- **Off-box measurement is allowed**: temporary GitHub Actions
  workflows may be used to take quiet timings while this 4-core box is
  running several lanes.

Opening commit: `prefix: perf/`, `tag`, band 3400–3499 claimed in
`docs/MODEL-AB-LOG.md`, this log. Exploration wave dispatched next
(Opus lanes, per Ev's budget steer): a GUI-seat lane on the edit→repaint
path, a kernel-seat lane over the demos and the Python binding, a
developer-seat lane over the debug/CI profile, and a static lane
re-verifying `plan.md` §1.3 against today's tree and the tracker's
already-filed perf findings. Each finding becomes one item file here.

## PERF-8: a composing door on the K-funnel (2026-09-12)

**Announcing a `geom-core` edit in PROPS' territory** (`work/README.md`:
the owner is told where the change lands, and this is that telling).
`crates/geom-core/src/k_stats.rs` gains two doors and the type between
them — `detached`, `splice`, `Detached` — and nothing else in that file
changes behaviour. `Bracket`'s semantics, nesting included, are
untouched; `detached` is built out of `Bracket` rather than beside it.

Why it had to be there: the funnel's recording is thread-local
(`FRAMES`, and `SINK` under `probe`), so a face decided on a rayon
worker records into that worker's frame and sink. `detached` runs a
closure on the current thread under a frame and sink of its own and
hands back what it recorded; `splice` appends such a recording to the
current thread's open frame and sink. A walk that maps faces onto
workers and splices in arena order therefore produces the serial walk's
verdict log, escalation log and sample population, element for element,
at any thread count.

`crates/topo/src/props.rs` is the first consumer (`mass_properties` and
`sign_certified`); `topo` gains the workspace `rayon`. The same door is
what the other four rayon maps in the tree need —
`work/perf/rayon-maps-outside-props-lose-the-funnels-recordings.md` and
`work/wire/parallel-node-map-loses-the-funnel-and-the-symbolic-session.md`.
## PERF-9 lands on GUI/VIEW ground (2026-09-12)

Unit `PERF-9` (branch `perf/9-per-face-bvh`) changes `editor-core`'s
index seam (`resolve/pick.rs`: the pick index is a tree per patch
under a tree over the patches, the per-patch trees memoised beside
the patch memo in `PickMemo`) and the viewer's seam report
(`evalseam.rs`: `MemoReport` gains the tree level), and adds two
read-only accessors to `crates/bvh` (`Bvh::boxes`, `Bvh::heap_bytes`;
no query or build changes). The pin is `viewer`'s `index_memo`
differential against a single-level reference, tie-break row
included. A finding on `docm`'s ground went to its slate:
`work/docm/pick-grazing-ray-answer-depends-on-candidate-order.md`.

## PERF-11 lands on PROPS/TOPO ground (2026-09-13)

**Announcing a `crates/topo/src/props.rs` edit in PROPS' territory**
(`work/README.md`: the owner is told where the change lands). Unit
`PERF-11` (branch `perf/11-props-walks`) executed the continuation half
of `face-walks-outside-mass-properties-are-still-serial` and MEASURED
the census half into a ruling:

- `SignCertificate::refine_to_target` resumes its open faces as an
  indexed map into one slot per face, each under a detached K-funnel
  frame, and walks those slots sequentially in arena order — so the
  refusal it names is still the FIRST refusal in arena order, resumed
  or already outstanding. The map is bounded to the slots before that
  refusal and is skipped entirely when no face is open (58 of the
  tour's 61 gated stops); a symbolic session keeps the serial walk.
- `classify_shells_of`'s per-shell face loop **stays SERIAL**. It was
  mapped, measured, and reverted: the map costs about 3× at four
  threads on the corpus heat sink at 41 and 161 shells, 4.6× on a
  two-shell voided brick, 2× on a hollow box, and gains on no body in
  the corpus, because no corpus shell is many-faced on the quadrature
  lane. What it kept is the dedup — the hand-rolled per-shell sums are
  `fold_runs` now — and the serial walk has one home
  (`decide_faces_serially`), shared with `decide_faces`' session arm.
  The grain that could repay the price there is the loop over SHELLS,
  which measures at about break-even on the same document; that is the
  census's own question and PERF-12's ground.

Pins: thread-count goldens for the census
(`crates/sweep/tests/shell_census_is_thread_count_invariant.rs`, cut on
the merge base — on a serial walk they pin that its readings and its
recorded channels do not read the pool at all), the continuation's
refusal-parity, piece-evaluation and session-receipt rows at an
explicit one and four threads
(`continuation_is_thread_count_invariant.rs`), and the refusal ORDER
itself on a hand-built certificate in `props.rs`
(`continuation_refusal_order_tests`), where no fixture body can put an
outstanding refusal ahead of an outright one.

Measured: the round spout's continuation through the tour 16.2 s →
4.1 s at four threads (14.2 → 14.7 s at one — the width is the whole
of the saving), the whole tour 24.9 s → 12.5 s at four threads;
`quintic_prism`'s continuation 8.3 s → 2.1 s. Evidence added to two
open items rather than new rows:
`quadrature-setup-is-re-derived-per-round-window` (the open-face census
over the tour's 61 gated stops, and the setup term's 8–99% share) and
`parallel-map-costs-a-fixed-price-on-a-cheap-body` (the census's
measured regression, the ruling it forced, and the shell grain).
