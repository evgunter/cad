# PR 3984 (the planar crossing lane refuses curved carriers, typed): LAST fix pass

The dual review is in, frozen at 8abb6e7931. Both reviews are APPROVE-WITH-FIXES with no MAJOR. The reports are on analysis/reach-dual/3984-r1 and -r2; read both. "A" and "B" below label the two reviews blindly.

This is the LAST fix pass. The orchestrator verifies it, running mutants and rows, and there is no further review round.

## Rulings
- **Scope.** The frozen head keeps `gate_operand_edges`, and both reviews reviewed that state. Keep the gate in this PR. Deleting it, which the NURBS fork's convergence called for once the three silent sites are typed, becomes its own follow-up unit: file it as a REACH item (P1) citing this PR. Do not delete the gate in this pass; doing so would put unreviewed behaviour into a last pass.
- **The PR title and body** must describe this head, not a later plan (A's claim finding).
- **The spiric half of the curved arm** (`reduce.rs:2181`) gets a row of its own that B's surviving mutant turns red. All the current rows are NURBS.
- **The duplicate variant and its recourse text** (`mod.rs:1601-1625`, `2840-2849`): give it one home. A showed that no public path raises it. If it is truly unreachable, say so at its definition and pin that, or fold it. Fix the "near a face" wording.
- **The gate's stale reason** (`reduce.rs:416-419`, `479`): rewrite it to what the gate does now.
- **The join's ring and germ-frame residue** (`join.rs:1036`, `1902`; `chord_join.rs:2430`): file it as an item with the citations.
- `touch_at_end` skipped for lines (`reduce.rs:1224`): disclose it in the PR body. The refusal class does not change.
- Items under "File rather than fix" are filed as work items, not fixed.
- Work under the intent-refactor hold (the d10 item), and don't widen the PR.
- Run the full battery at all three ε, and the Python suite and binding census. Don't merge, and post no comments. Update the PR body with a map from finding to change.

## 5. Consolidated fix list (deduplicated, by severity)

**MINOR (fix in this PR):**

1. **F6, pin the spiric half of the curved arm.** Add a spiric row to `planar_lane_carrier_rows.rs`: B's P2, a spiric cap
   against a cylinder wall at ρ = 2.5. Optionally add P1 (a spiric dipping through the brick face y = 2.5) for the planar
   arm. The row must go red when `reduce.rs:2181`'s `Spiric` alternative is routed to `frontier()`.
2. **F1, fix the gate's reason.**
   - Rewrite `reduce.rs:416-419`: drop "what the zip MINTS rather than what it consumes".
   - Reword `reduce.rs:479`: the sweep arms now read a spiric, but only to refuse it typed.
   - State which downstream sites the gate still guards: the germ frame (`join.rs:1036`), the ring run (`join.rs:1902`,
     `chord_join.rs:2430`), the sector chord, section area and the continuation scan.
   - Update the matching comment at `test_north_star.py:3504`.
3. **F7, rewrite the PR title and body to describe 8abb6e7931**, or re-freeze on the head the body describes. As written,
   the body claims a gate deletion and `EdgeCarrierUnsupported` that this head does not contain. Cite run 37141807328.
4. **F2, decide the variant's standing.**
   - Either say in its doc (`mod.rs:1610-1617`) that it is unreachable while the gate stands and exists for the narrowed
     gate, and give the planar-lane rows a line saying when they become live.
   - Or drop it in favour of `CurvedEdgeUnsupported`.
   - In both cases, de-duplicate the recourse text at `mod.rs:2840-2849`: share one constant, or reference the other
     variant's sentence.
5. **F3, file the unscheduled residue.** File a `work/reach` issue for the `join.rs:1036` `JoinDesync`, `join.rs:1902` /
   `chord_join.rs:2430` `SectionInvariant` and continuation-scan sites. Each still says "the operand gates refuse the
   kinds", and each would mislabel a typed carrier refusal as a kernel invariant once the gate narrows. Reference it from
   the unit item.

**Style and NOTE (cheap, fix in this PR):**

6. **F10:** replace the hand-built `geom_brep::Conic` at `classify.rs:466-485` with `Conic::of` (`implicit.rs:662`).
7. **F11:** remove the unreachable arm at `reduce.rs:2188-2192`, for example by matching on `Conic::of(carrier)` first.
   This follows naturally from item 6.
8. **F12:** inline or remove the `crossing_lane` wrapper at `classify.rs:291-299`.
9. **F5:** fix the wording at `mod.rs:2845`. For example: "an edge … whose box meets a face …"; for a spline the box is
   the whole space.
10. **F14:** correct the doc at `classify.rs:942`. The spiric arm does not cross y = ½; say the row is kind-keyed.
11. **F15:** update the stale premises at `boxes.rs:1774-1778, 1825` and `contain.rs:248`. Sweep soundness past the gate
    now rests on the spiric box pruning.
12. **F13:** update the citations of `conic_plane_crossing_roots` in the four work items and in `GERM-VERBS-CONE-SPEC.md:119`.
13. **F4 (optional):** add one line to the PR body saying a line's endpoint sides are now decided only by the line lane
    (`reduce.rs:1393-1403`). The refusal class is unchanged.

**File rather than fix (pre-existing or off target):**

- **F8:** pin, or remove, the duplicated `edge_clears` arm in the split's `insert_crossings` (`classify.rs:734-739`) behind
  `gate_operand` (`classify.rs:73-76`). A's mutant M5 survives all rows. Pre-existing.
- **F9:** rod∖brick (d = 0.0137) ∖ slab z ∈ [3.5, 4] refuses "the solids do not cross" despite the overlap, and
  `point_in_solid` answers `WallOutlineUnsupported`. Identical on main; the cause is unmeasured.
- **F13, the core's home:** the wrongly homed core (`work/pipe/topo-shared-cores-hosted-in-one-half.md`) is widened by
  this PR. Note it on that P1 item rather than move the core here.
