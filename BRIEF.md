# Reviewer brief: SECT PR 6153 (the torus rows read at 1e-6 again)

You are the single FULL reviewer (claims to falsify plus the style lane).

**Frozen head.** `96368e32997a5d6316d65b6cae3793e431561383` of https://github.com/evgunter/cad/pull/6153
(branch `sect/torus-rows-at-1e-6`): `git fetch origin sect/torus-rows-at-1e-6 && git checkout --detach 96368e32997a5d6316d65b6cae3793e431561383`.
Read the PR body via the GitHub MCP tools (ToolSearch; no gh). Do not read its
comments or reviews. Post nothing, push nothing.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md`; `CLAUDE.md`;
C2 (limbs 1–3), C5 and C7 in `crates/geom-brep/README.md`; the row
`work/sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot.md`
as on main; `work/sect/a-general-torus-pose-lists-no-components.md`
(`## Decided`, `## Closed`).

**Builds.** `CARGO_TARGET_DIR` outside the checkout, `CARGO_INCREMENTAL=0`;
opt-level 2 for marching rows; foreground; anything past 600 s
`setsid`-detached and polled; nothing left running. These are slow-set rows
the per-PR gate does not run: run them yourself (`--run-ignored all`) at the
default ε, 1e-6 and 1e-12 (`CAD_TOLERANCE_EPS`). **Budget: about 3 hours.**

## Claims to falsify

1. **Winding over the tube boxes** (`pair_section/winds.rs`, new
   `ssi::certify::tube_chain`): limb 3 proves one arc with a piece in every
   box, so the component and the carrier pass through the same convex boxes in
   order and wind alike, with no closeness bound and no halving. Is that the
   lemma limb 3 actually proves (boxes in order? overlapping or adjacent? does
   the arc visit each box once?), and does winding about the axis or core
   follow when a box straddles the axis or core? Falsify with an independent
   winding count (dense sampling of the true section) on random transverse
   poses, near the axis and core, near tangency, at all three ε; check
   `Untiled` is typed and raised only where the pieces really don't tile.
2. **The pad** (`Tube::of`): the tube's farthest point from the carrier,
   bounded per slab (8 slabs × 9 interval carrier samples). Is that an upper
   bound (between samples, at slab ends, for the cubic box padding)? Try to find
   a point of a tube box farther from the carrier than the pad.
3. **W3's `reach`** (`CarriedComponent::reach`, passed as
   `section_cert::Component::pad`): the diameter of a tube box holding the
   witness. Does it bound the witness-to-component distance the certificate
   needs?
4. **Per-row decisions**: four rows un-pinned (two stop where main stops them
   at the default ε, on GREEN's open P0
   `main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps`);
   `close_rings_carry_counted_and_marched` stays pinned on a march defect
   filed on SSIMARCH. Reproduce at 1e-6 and check each decision. Is
   `two_loops_keep_their_sites_and_order_near_the_node`'s thin margin (pad
   ~0.10 m against a sheet 0.10–0.21 m away) a pass that a nearby pose would
   turn into a wrong reading rather than a refusal?
5. **No verdict moved at the default ε or 1e-12** across PR 6011's touched rows
   (the PR's 444/461-test table): reproduce on head and merge-base.
6. **Suites:** `geom-brep`, `topo`, `sweep` (`--profile ci`) on head and merge-base.

## Report

Final message, ≤150 lines: verdict (APPROVE · APPROVE-WITH-FIXES ·
NOT-MERGEABLE-AS-IS · REJECT); findings MAJOR/MINOR/NOTE with `file:line`,
evidence, executed or by inspection; `## Style`; tests worth keeping as code
(commit nothing).
