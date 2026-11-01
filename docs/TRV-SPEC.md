# TRV: the clearance engine refuses a bracket that does not certify

Unit: `work/clear/clearance-lifts-trv-brackets-and-reads-their-ends.md`
(P1/M). Program: CLEAR. Branch: `clear/trv`. Review: a single FULL
review with mutation.

Read the row in full. Its line numbers are from `0011f58a`, so re-find
each site on current main. Its per-read reading is a census lane's, and
the row asks you to check each claim before fixing it.

## What is true when this lands

1. **R1: refuse at the lift.** In `boundary_of`, `window_of` and
   `RegionMemo::ask` (`crates/topo/src/clearance.rs`), a value lifted
   through `Interval::from_certified` that does not certify is refused.
   The refusal is `None`, or `Unsupported`, or the arm's existing typed
   refusal. The same question is already asked in `extent_of` and
   `iso_bounded_domain`; make the sites agree.
2. **The reads the row calls unsound are each made sound, or shown
   sound with the argument at the site.**
   - **clearance.rs:**
     - `min_separation`/`Bracket::pair`: poison on
       `!sep.is_certified()`, not only on non-finite ends.
     - `Bracket::witness`: skip a station separation that does not
       certify.
     - `Bracket::overflow`: an abandoned task's refused `sep` poisons,
       or floors at 0.
     - The `window_of` and `boundary_of` reads, through R1.
   - **injective.rs:**
     - The 10 reads that need R1 and a local ask: `forward`, `along`,
       `present_chart`'s merge hull, `Trig::psi`'s `over`, and
       `Curved::psi`'s bound.
     - The 2 reads that are unsound whatever the lift:
       `present_plane`'s `[e]` arm `y.hi() <= 0.0` before the funnel;
       `present_chart`'s sliver minting a width from a refused lever.
     - The `f64::max`/`min` merge folds that skip a NaN clip line.
   - Where you find the row's reading wrong for a site, say so with the
     argument, and leave the site.
3. **The census.** Re-pin `geom-core`'s `certified_endpoint_census`
   (`crates/geom-core/tests/certified_endpoint_census.rs`, `ROSTER`)
   for `injective.rs` and `clearance.rs` at the new counts. Replace the
   two notes that name this row with dispositions.
4. **Rows. Each can go red.**
   - At least one row per class (R1 at the lift, a poisoned separation,
     an unsound injective read) that feeds a `Trv` or refused value and
     shows a typed refusal where today the engine decides on it. Build
     the input by hand, or with a poisoned surface or pcurve. If no
     supported input reaches a site, say so at the site.
   - Every self-intersection, pipe, cavity and clearance row, and the
     editor-core `m10_6_min_clearance_interval` row, keeps its verdict
     class at both scalars. f64 receipts are bit-identical; list any
     moved Interval receipt.
5. **Mutation.** Each added ask, made lax, reds its row.

## Ground

- `crates/topo/src/clearance.rs`, `crates/topo/src/injective.rs`,
  `crates/geom-core/src/interval.rs` (`from_certified`, `Trv`) and the
  census test.
- HEADING (`clear/heading`, `graph_of`), ZEROSIDE (`clear/zeroside`,
  `sign_on`/`centred`) and SLIVER (`clear/sliver`, `rect_jet`) are in
  flight nearby. Merge main as they land.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤100 lines, covering:
- R1;
- each read, with its fix or the argument for leaving it;
- the rows;
- the census re-pin;
- the mutants;
- the PR number and its CI result.
