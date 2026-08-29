# Work-stream proposal — 2026-08-29

Non-binding planning survey: candidate streams of work, each scoped to
one topic and cut to avoid the territory live programs and in-flight
PRs already occupy. A stream graduates by opening as a program (PLAN /
LOG pair, one short branch prefix, next free A/B ordinal band — 600+ —
recorded in `docs/MODEL-AB-LOG.md` in the opening commit) or by folding
into the program that owns its territory. Inputs: all 152 open issues,
the open PRs, and the live program docs, all read on 2026-08-29.

**Revised 2026-08-29 (later the same day): PCURVE closed.** Its exit
walk is ratified, its whole slate is on main, and the `geom-brep` and
`topo`-consumer ground three streams fenced against is released. The
PCURVE bullet below, S-BOOL's and S-MESH's track claims, S-CERT's
keep-out list and the overlap rules are updated to that state; nothing
else in this cut moved.

## Territory already occupied

- **M10** (`m10/`): editor-core parameters/analysis/eval, `geom-core/dual.rs`,
  the `AtRestPolicy` seam in `topo/src/props.rs` and the Dual arms of
  `editor-core/src/product.rs` (M10-4), the conservative interval lift
  of `crates/bvh` (M10-5), schema v15/v16, profile-parameter lift
  (#1174 under dual review), M10-2 Measure/Assertions spec just
  merged (#1197).
- **PCURVE** — **CLOSED (2026-08-29)**, and its territory is
  **released**. `docs/PCURVE-EXIT-WALK.md` is ratified and is the
  program's done-state of record; P-1a (#1073), P-1b (#1107) and P-2
  (#1177, merged at `fec15263`) are all on main, and P-3 was removed
  before any code was written. The `geom-brep`
  certify/edge_nurbs/adopt/nurbs_iso files and the `topo` pcurve
  consumers are nobody's fence now — see S-BOOL and S-MESH, both of
  which fenced against them and no longer need to.
  `Vec3::orthonormal_basis` (#1157) **is fixed on main**: the
  denominator is written `1 + |n.z|`, and the partial first fix (which
  the f64-bitwise row structurally could not catch — exit-walk H2) is
  closed by `orthonormal_basis_is_bounded_over_z_enclosures` beside
  it. **The issue is open as bookkeeping only**; do not schedule it as
  work without re-reading `crates/geom-core/src/linalg/vec.rs` first.
  What the walk leaves **named and unowned**, none of it cut into a
  stream here: #264 (`PXN_IMAGE_DEGREE`, which blocks the split-out
  diagonal half), #1195 (de Boor collapse extractor), and #1179 (the
  six measurement consumers that now refuse TYPED on an affected
  face). Its other named residues already have owners — #1152 is
  S-BOOL's below, #1143 is M10's (governed by ratified DL6),
  #1128/#1204 are S-QA's.
- **VERBS** (`verbs/`): shell/offset solves in `crates/sweep` and
  `topo/replace_face.rs` (SHELLFIX 2b, #1180, in its final fix pass);
  its post-SHELLFIX queue is boolean breadth — germ arms (#347's
  remaining half), SPHSPH, the C5 section arms (#1057), CYLSPH, with
  #1031 half B, #1076 and #1077 claimed alongside (per Evan's ruling,
  recorded on this PR's thread; specs staged in the VERBS drafts dir).
  The ratified DRAFT/MIRROR designs stay its backlog.
- **LIB** (`lib/`): pncad façade, pncad-py, guide pages (#1198 open);
  owns refusal-display prose and the #741/#742/#944 plans.
- **SMELL** (schedule in `SMELL-SCAN-2026-08.md` §D): tracks K, P, W
  have a live session with unlanded rows; K's bounds-allowlist rows
  (D102/D103/D106/D109) are contested by live branches.
- Parked but reserved: fillet-seam CertifiedBounds (#883 → lane H-f).

Most contested files: `crates/geom-brep/`, `crates/sweep/`,
`crates/editor-core/`, and schema-version bumps (M10 holds v15/v16).

## Proposed streams

### S-CERT — certified-enclosure soundness (`cert/`)

**GRADUATED (2026-08-29): opened as the S-CERT program —
`docs/S-CERT-PLAN.md` / `docs/S-CERT-LOG.md`, A/B band 700–799.**

The wrong-but-green and uselessly-wide certificate cluster — the
largest real-defect group in the tracker and nobody's territory.

- Accepting defects first: #723 / #893 (sphere meridian/rim certify
  wrong volumes near poles; S82 feeds the same conversation). VERBS'
  staged SPHSPH unit inherits neither, plants the missing near-polar
  red, and stops if acceptance needs the props fix — so these fixes
  sit on that unit's critical path.
- Interval-mode honesty: #924 (rotation anchor width), #1191
  (period-fold widening — M10-P filed it and deliberately did not
  repair it; the stream takes it under the issue's stated f64-bit
  constraint, and M10-3's driver is its first heavy consumer), #762
  (chart-speed guard admits +∞).
- Enclosure quality and metering: #870 (area never metered),
  #453 / #390 (one rational-patch-flux lane, native and import sides),
  #528, #501, #303, #1006; the offset_fit sub-family #1005–#1008.
- Claims SMELL tracks M (`geom-core` scalars/bvh) and N (`crates/geom`
  spline/linalg) — same files, no live claimant. The track-M claim
  carves out `crates/bvh`'s interval lift (M10-5's); bvh work here is
  f64 box quality only, anything past that coordinates with M10 first.

Keep out: `geom-core/dual.rs`, the `AtRestPolicy` seam in
`topo/src/props.rs`, the Dual arms of `editor-core/src/product.rs`,
and Dual-at-certified-gates semantics (all M10's ratified slate);
#1018–#1020 (scheduled at OFF-D under VERBS); the #1143
poison-vs-widen ruling (M10-D ratified the contract; this stream
supplies instances, not the answer).

### S-BLEND — fillet/chamfer completion (`blend/`)

Blend-verb breadth, disjoint from SHELLFIX's shell/offset files.

- Fillet reach: #1022 (seam-split closed rim), #987 (ruled-spine
  carve), #935 (two rims sharing a wall), #644 (convexity-parametric
  corners), #961 (RimSupport vocabulary), #708 (N2 tie propagation).
- Chamfer parity: #918 (recipe-layer door), #919 (concave chains),
  #917 (shared vocabulary speaks as the fillet).
- Design conversation to open with Evan: #827 (enclosing tangency).
- Claims SMELL track T (`crates/sweep`, 10 rows) once SHELLFIX 2b
  merges; coordinates ordinals with VERBS or opens its own band.
- Handoffs from VERBS: #1022 builds to the corrected A3-2 record in
  `docs/ARMS3-DESIGN.md` (the tag's promised recourse is measured
  impossible without the multi-link door — build to that record, not
  the issue's original framing); #827 starts from the JunctionTangent
  pin in `lily.rs`'s `review_probes` (margin 1.6e-17).

### S-BOOL — boolean reach and containment (`bool/`)

Operand gates and containment doors that refuse (or mis-admit) legal
inputs — `topo/boolean`, `splitting`, containment; not pcurves, and
not the germ-arm lanes VERBS holds. VERBS' claims (its Wave-4 queue):
#347's remaining half, #1031 half B, #1076, #1077; #1059 is resolved
into #1031's chain (register row, LILYWELD close) and drops from the
cut.

- Gates: #1011 (`point_in_solid` missing ray arms — the named cost of
  VERBS-GATE's pair-scoping), #1152.
- Containment/props: #750 (extent-box coarse), #542, #368, #433
  (needs a disposition), #134.
- **Claims SMELL track Q whole** — the topo rows and, since PCURVE
  closed, the four `geom-brep` paths Q's fence names (`ssi*`,
  `pcurve_cache.rs`, `nurbs_iso.rs`, `edge_nurbs.rs`). The earlier cut
  held those back for P-2; nothing fences them now, and taking the
  whole track is what the partition's single-owner rule wants. That
  adds D36, D283 (a design question for Evan, not a lane's) and D284
  to this stream, and the geom-brep halves of D46, D281 and H11.

### S-MESH — mesh honesty and budget (`mesh/`)

`crates/mesh` has no live program and a coherent defect list.

- Watertightness/guards: #897 (S65's two uncovered cases — S65 itself
  is Evan's), #896 (undeclared-pole misclassification), #868 (typed
  warning channel).
- Sizing intent vs budget: #685 (`nv` ignored), #320 (NURBS wall
  budget, per `memories/tessellation-budget.md`), #950, #555.
- Structure: #881 (ε as a bare f64), #726 / #727 (iso-rectangle
  ownership — design input), #782 (red tessellation pin, unrun by any
  lane; re-baselining is a mesh question).
- **Claims SMELL track R whole** — the mesh rows and the `geom-brep`
  remainder, which PCURVE's close releases. That includes the ground R's
  fence took on 2026-08-28 (`patch_bound.rs`, `offset_meters.rs`,
  `nurbs_hull.rs` and the rest of the crate's `src/` less Q's four
  paths), so `C23` — whose constant is
  `patch_bound::RATIONAL_CERT_SPLITS` and which no track could execute
  before that widening — is takeable here. **Two of the released rows
  stay gated, and not on PCURVE**: `C3` and `D30` (`props/quad.rs`'s
  four quadrature engines) are NOT takeable until #723 is fixed, which
  is S-CERT's first accepting-defect row — consolidating first bakes a
  wrong certified volume in. Coordinate the order with S-CERT.

### S-QA — gates that lie (`qa/`)

**GRADUATED (2026-08-29): opened as the S-QA program —
`docs/S-QA-PLAN.md` / `docs/S-QA-LOG.md`, A/B band 800–899.**

The meta-cluster: test and CI infrastructure that reports green
without looking. Distinctive, urgent (main is red at one matrix
point), and touches files no kernel program is editing.

- Red now: #1102 (eps=1e-12 census row bites the next PR to draw it),
  with #1128 (fail-fast under-reporting) as the amplifier.
- Silent passes: #888 (`|| true` masks grep exit 2), #1023
  (filter-skipped lint), #1038 / #746 (tess-lint stops comparing),
  #1122 (filename-substring lane pin).
- Test integrity: #882 + #1134 (one panic-hook race, two issues),
  #774 (wrong generator), #651 / #681 (unguarded measured claims;
  #808 stays parked on #763), #470, #466.
- Operability: #1051 (workflow_dispatch matrix point), #469, #1139.
- Claims SMELL track J (workflows/scripts/`*.py`); coordinates with
  the live K/P/W session on `scripts/gates/` and stays off K's
  contested allowlist rows.

### S-MATE — contacts, rest, and assembly composition (`mate/`)

ASM is closed; its exit-walk residue plus the declared-contact gaps
form one topic with no live claimant. Ruling-heavy: this stream opens
as design conversations, then implements.

- Needs Evan first: #945 (mates × patterns), #943 (face-level Rest
  closure), #795 adjacent demo questions stay out.
- Implementation-ready: #946 (sub-assembly mate loss at the
  instantiation seam), #944 (mint alignment frames — LIB holds the
  plan; take only with LIB's hand-off), #1032 (cylindrical-only Rest),
  #973; #968 (torus declared-Rest) is ceded by VERBS with the
  #966/#968 record and LILYWELD's measured cone×torus adjacency —
  pickup stays governed by its ruling's conditions.
- Kernel side: #941 (declared cusps, ruled), #750 overlaps S-BOOL —
  assign to S-BOOL, consume here.

## Surveyed and deliberately not cut

- **DRAFT / MIRROR verbs, Wave 2 germ rows** — VERBS' own backlog;
  cutting them elsewhere would split that program's register.
- **Refusal-display / API-gap cluster** (#1111, #985, #947, #694,
  #561, #757–#759, #796, #948, #1103) — LIB's charter; it is actively
  landing exactly this.
- **Q3 sketch solver** — consumer-gated by Evan's ruling; re-opens as
  its own design pass, not a stream.
- **DISCIPLINES / PARAM-LINT** — next unit exists (`PARAM-LINT-SPEC.md`)
  but is a draft awaiting Evan's sign-off; blocked, not unclaimed.
- **GUI first light** (#1097) — needs real hardware, not a stream.
- **Ordinal/bookkeeping issues** (#1140, #1159, #1188, #1016, #614,
  #607, #430, #600, #250, #226, #214) — program records, not work to
  schedule.

## Overlap rules the cut respects

- One file territory per stream, mirroring the SMELL partition rule;
  where a stream claims a SMELL track it takes the whole track so the
  schedule stays single-owner.
- Streams stay out of SHELLFIX's shell/offset files, M10's
  editor-core/dual/schema territory, and LIB's façade/bindings until
  those units land; seams are named per stream above. (PCURVE's
  `geom-brep` files were on this list and are not any more — the
  program closed 2026-08-29.)
- Design rulings stay with Evan: a stream may file instances and open
  conversations (#827, #433, #726/#727, #945, #943) but not resolve
  them by implementing.
