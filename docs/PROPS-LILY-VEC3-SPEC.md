# PROPS lily-vec3 — the lily authored in `Vec3<f64>` through the kernel's own doors, lifted at the boundary

**Binding at dispatch** (PROPS program; the item is
`work/props/lily-authoring-needs-shadow-vector-algebra.md` — read it in
full, §Decided is the decision this executes; difficulty logged at
spec: **E**). An E rider: single style review, outside the A/B
experiment. Read `docs/prompts/implementer-discipline.md` in full (§3
Demos binds hardest here: the tour is evidence, written the way a real
user would write it) and `memories/demo-purpose.md`. Branch
`props/lily-vec3`, cut from `main`.

## What

`demos/tour/src/lily.rs` authors its geometry in bare `(f64, f64, f64)`
tuples with hand-rolled algebra (`rot`, `nrm`, the two radial-frame
builders, `blade_frame`, `v_sub`/`v_dot`/`v_cross`/`v_len`, `v3`/`pt3`
as the last-moment lift). Rewrite the AUTHORING half so the scene is
composed in `Vec3<f64>` / `Point3<f64>` with the kernel's doors —
`normalize`, `dot`, `cross`, `reject_from`, the operators,
`Mat3::rotation_about(Vec3::unit_y(), a)` for the planar rotation —
and lifted to `S` at the API boundary through `.map(S::from_f64)`
(`Vec3::map` / `Point3::map`). Delete every tuple helper that the
rewrite empties. Keep the `f64`-at-authoring layer: that is the right
layer for literal-heavy scene description under a generic `S`, and the
header says so in one paragraph (the invariant, not the history —
discipline §4): *scene data is composed at `f64` in the kernel's vector
types and lifted once at the door.* The narration/measurement half
(`Circle`, `TorusCarrier`, the `v_*` family at ~:1724-1740, and
`mod review_probes`) is rewritten through the same doors where it
duplicates one (`p / p.norm()` → `normalize`, `a − axis·(a·axis)` →
`reject_from`); its structure is otherwise not this unit's.

**Byte-for-byte is the acceptance for the geometry**: the tour's lily
scene must produce the same bodies. Where a respelling changes an `f64`
result by rounding (a different association in a normalize or a
rotation), measure the difference at the scene's outputs (the
`review_probes` rows and the committed tour renders are the
instruments) and say so; a moved render is REPORTED by CI and
re-baselined on `main` (`memories/freecad-render-lane.md`), never
adjusted to restore a frame.

**Not this unit:** any new door on `Vec3`/`Point3`/`Mat3` (the census
found none missing — if you find one, report it in the PR body rather
than adding it); the 118-line header essay (`D79`(f), Track X's, parked
on `L2`); other tour scenes (`az.rs`, `letterforms.rs`, `skinned.rs`
spell their own lifts — report them as members of the same layer rule,
do not edit).

**Fence:** `demos/tour/src/lily.rs` only (Track X's ground by
`work/code-quality/D79.md`; the unit is the check of a kernel question
this program owns, and `D79`(b) is closed member by member at landing —
edit `work/code-quality/D79.md`'s (b) bullet and the `S130`(b) text it
mirrors, by deletion, citing this PR; no other program directory).

## Posture

- `cd demos/tour && cargo test --release` (the gated probes) and the
  tour's render row green on hosted CI; clippy and fmt on the tour's
  own cargo root (it is an excluded root — build it the way the gates
  do; `scripts/gates/` lists them).
- No `CI-Config:` trailer; no `Co-Authored-By`; `work.py lint` green;
  the item gets `pr:` and `status: review` on this branch; the spec is
  deleted at merge.

## Acceptance

Zero tuple-algebra helpers left in the authoring half; the lift spelled
once per boundary through `map`; the header paragraph; the geometry
byte-identical or every rounding difference measured and stated; `D79`(b)
deleted member by member.
