# Design question: on a fat torus, what guarantees that a section component whose only events are vertex contacts is joined truly or refused, and which layer owns that guarantee?

You are a DESIGNER for the `evgunter/cad` repository, a Rust B-rep CAD kernel. Read `docs/prompts/designer.md` and follow it: it is binding, and it sets your report's shape, including the "For Ev" and "For the orchestrator" sections. Also read `CLAUDE.md` and `docs/DESIGN.md` (D1–D10; D10's hold, `work/intent/d10-one-way-to-say-intent-is-unbuilt.md`, matters here). Work from main. You may read any code and run anything.

**Do not read** the open-question entry "Q1" in §4 of `docs/GERM-TORUS-PLANE-FRAME-SPEC.md`, nor the "the lens (U-T4)" row of that spec's §2 table: they contain candidate answers, and this statement deliberately gives you none. Read the rest of that spec freely (§0–§1.4, §3's U-T4 rows).

## The problem
The plane × torus join (GERM's units U-T1–U-T3, merged; PR 4847 is the latest) builds bodies for a plane cutting a torus in meridian or concentric circles. U-T4 will add the chord for **spiric ovals** (a plane parallel to the axis, short of the inner equator, cuts the torus in two ovals).

Spec §1.4 measures a configuration that the spiric chord makes reachable: a **corner bar** (a square bar of half-width `w` along the torus axis, its corners on the tube's inner face) against a **fat torus** (`R < r + s`, `s = √(r² − w²)`, so `R < 2r` is necessary). Fixtures B5 (`R = 1`, `r = 0.6`, `w = 0.1`, end squares at `z = 0.396` against `R − r = 0.4`) and B5′ (`w = 0.3`).
- Each end square of the bar cuts two spiric ovals. Each oval runs from corner to corner across the square, so the square holds a **lens** of the tube on each side, bounded by oval arcs whose only events (germs) are the bar's corners, where the bar's edges pierce the torus.
- The frame (`pt_germ_frame` / `pair_section_frame_at`, `crates/topo/src/boolean/join.rs`) serves those ovals, and every op passes the frame and the dispatch and stops at F3 (the chord), today a typed `SectionUnarmed`.
- The section certificate (`crates/topo/src/boolean/section_cert.rs`; its witness rules W0–W4) clears those section components as `Essential` (W2), so **nothing upstream of the chord guards the lens**.
- On the donut (B4, `R − r − s ≥ 1`) the frame refuses the corresponding end squares as a null oval (`GermSectionSpiric { loops: Null }`), pinned by `germ_interior_oval::the_corner_bar_never_comes_back_a_body`.

Once the spiric chord lands, a body built here either includes the lens correctly or misses it. The spec's U-T4 obligation (§3, U-T4 row 3) is: B5 and B5′, all six ops, each either the true body (volume, tier 3, `point_in_solid` against the truth) or a typed refusal, **never a wrong body**, and red against the mutant that chords the end square along its own edges, skipping the oval arcs between the corners.

**The question:** what mechanism makes that obligation hold, in which layer (the join's germ pairing and chord, the section certificate, the frame, or elsewhere), and what does that choice make true or false for other surface pairs where a section component's only events are vertex contacts (a corner touching a curved face)? This includes whether such a component should be answered at all, and what the measured cost is of each answer on poses that build correctly today.

## What to measure, at least
- Which poses on main today have a section component whose only events are vertex contacts, across every surface pair the certificate serves, and what each currently returns (a body, true or not, or a refusal). This has not been measured.
- B5 and B5′ (and a sweep around them: `w`, `R/r`, tilt) against the truth, with your own oracle of the lens.

## What to deliver
Follow `docs/prompts/designer.md`. Recommend a final state, with what it makes true, what it costs (which legal poses it refuses or changes, and which layers or other programs' modules must be kept in step), and how sure you are. If the framing above is wrong, say so. Note any ratified text your answer would change.
