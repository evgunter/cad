# `sweep` — solids from profiles, and the edge blends

`sweep` turns a `profile::ValidatedProfile` into a closed B-rep solid
(faces on stored surfaces, edges on stored curves, glued by half-edges)
and blends the edges of one. The sweep verbs are extrude, revolve, loft
and path sweep, all driving `topo`'s certified Euler operators over one
shared lowering. The blend family is the constant-radius rolling-ball
fillet and the equal-setback chamfer: one validity battery run over the
inputs before any surface is minted, one table of analytic arms, one
in-place composition surgery that splits the supports along stored
trimlines and grafts the band in, one birth record per minted entity,
one verb-neutral refusal vocabulary. Nothing is sampled or approximated.

The fillet design proper (the battery's six predicates in binding
order, the arm table, the corner-configuration scope) is in
`crates/geom-brep/README.md` under CURVED-DESIGN C8. What is not yet
built (run-outs, the canal-surface blend, curved-support chamfers, the
ruled-spine carve) is registered in `docs/KERNEL-VERBS.md`; the canal
blend is `docs/DESIGN.md` frontier (f).

## Where in the code

| Concern | Module |
|---|---|
| Extrude: translational sweep along the sketch normal; caps, walls, hole rings, rim upgrades | `crates/sweep/src/extrude.rs` |
| Revolve: axis and angle conventions, partial wedge and full ring, seam meridians, the `tube_along_arc` torus door | `crates/sweep/src/revolve/` (`mod.rs`, `axis.rs`, `partial.rs`, `full.rs`, `chain.rs`, `surfaces.rs`, `upgrade.rs`, `tube.rs`) |
| Loft and path sweep bodies: extrude's topology over NURBS walls | `crates/sweep/src/loft.rs` (`loft_body`, `sweep_body`) |
| Skinned and swept NURBS surfaces (the definitional geometry, not an approximation) | `crates/sweep/src/skin.rs` |
| The lowering every profile sweep shares: traversal order, carriers, cosurface decisions | `crates/sweep/src/swept.rs` |
| The two blend doors, one per verb; each attaches its `BlendKind` once | `crates/sweep/src/fillet.rs`, `crates/sweep/src/chamfer.rs` |
| Shared blend vocabulary: `BlendKind`, `BlendRefusal`, `BlendError`, `CornerConfig`, `RunOutPolicy`, recourse sentences | `crates/sweep/src/blend/mod.rs` |
| Validity battery, per-verb predicate gating, arm dispatch (`coaxial_arm`), `is_seam_vertex` | `crates/sweep/src/blend/battery.rs` |
| Analytic arms: sheet derivation, `BlendArm`, chamfer strip, corner ball | `crates/sweep/src/blend/arms.rs` |
| Admission tokens (holding the value is the fact) | `crates/sweep/src/blend/admit.rs` |
| Assembly front doors, `Blended` result type, octant charts | `crates/sweep/src/blend/build.rs` |
| In-place composition surgery; open chains, ladder rims, annulus rims across seams | `crates/sweep/src/blend/surgery.rs` |
| Birth records (`BlendNaming`) the document layer turns into names | `crates/sweep/src/blend/naming.rs` |

## Blend vocabulary (BLEND-VOCAB-DESIGN V1–V4)

The fillet and the chamfer are one request over the same bodies, judged
by the same predicates and carved by the same surgery; only the band
differs. They share one error type; these decisions say how a shared
refusal names the verb that raised it.

**V1 — the verb crosses as one wrapper at the door.** `fillet_edges`
and `chamfer_edges` return `Result<Blended, BlendRefusal>`, where
`BlendRefusal { verb: BlendKind, error: BlendError }` is minted once by
the door's `map_err` and nowhere below. The recipe layer's
`NodeErrorKind::Blend { verb, error }` (`crates/editor-core/src/eval/`)
is the same wrapper one layer up, filled from `BlendRefusal::verb`. One
discrimination point per layer: no verb field on the variants, no
per-verb enum.

**V2 — inner prose is verb-neutral; the wrapper supplies the verb.**
`BlendError`'s `Display` and the shared `FILLET3_*_RECOURSE` constants
never name a verb; `BlendRefusal`'s `Display` prefixes `"fillet: "` or
`"chamfer: "`. A recourse is a claim about a second request, so every
shared sentence is true under both verbs; a door one verb lacks is
conditioned in the sentence (the closed-chain clauses of
`FILLET3_ASSEMBLY_RECOURSE` and `FILLET3_SEAM_VERTEX_RECOURSE` say a
chamfer has no band). Ball-only arms keep ball language because no
chamfer run reaches them: `RadiusHeadroom` and `SpineIrregular` are
metered only when `run_battery_for` sees `BlendKind::Fillet`, and
`SpineUnsupported` sits behind the chamfer's early return
`ChamferArmUnsupported`.

**V3 — the shared machinery is blend-named; per-verb doors are thin.**
The module is `sweep::blend` (`BlendRequest`, `BlendError`,
`BlendNaming`, `blend::surgery::blend_surgery`); `sweep::fillet` and
`sweep::chamfer` re-export their door and result alias and nothing
else; ball-specific identifiers (`corner_ball`, spine and torus
language) keep their names. Three fences stay fillet-named on purpose:
the `fillet3_*` predicate names (a K-corpus family; renaming would fuse
telemetry buckets), the persisted `RoleSeg`/`RimSupport` role vocabulary
in `crates/editor-core/src/names/`, and `OpGroup::Fillet` there, whose
name under-describes what it groups (the chamfer reuses the same roles;
the minting node tells the two apart).

**V4 — no parallel enum.** There is no `ChamferError`; minting one is
not an admissible way to discriminate the verbs.

Settled choices: "edge blend" is the neutral generic noun; `Blended<T>`
is the one result type of both doors, `Filleted<T>` and `Chamfered<T>` its call-site aliases.

## Fillet arms and the seam vertex (ARMS3-DESIGN A3-1…A3-3)

**A3-1 — the sphere×sphere arm is a row of the coaxial family.** Two
spheres on distinct centres always meet in a circle whose axis is the
line through the centres, so the pair is coaxial by construction and
`fillet3_support_coaxiality` is zero there rather than measured.
`coaxial_arm` maps `(Sphere, Sphere)` to `BlendArm::SphereSphereTorus`;
the centre is the crossing of the two offset circles in the meridian
sheet through the rim (`Meridian::trace`), material sides read from
each face's stored sense bit folded with the chain's convexity verdict
(the ball rests on the material side of each support on a convex
chain and in the void on a concave one — S10/S11, the one fold every
arm spells, `plane_plane_blend`'s `signed` its precedent). A
tangential pair poisons the spine radius and escalates at predicate 3
(`spine_regularity`). The band carves on either material side: a
lentil's convex equator loses material to it, a two-sphere snowman's
waist gains it, through one surgery.

**A3-2 — a valence-4 seam vertex is not a corner.** Where a chart seam
(the `u = 0` meridian of a revolved wall) crosses an otherwise smooth
latitude rim, the vertex has valence four: two rim arcs carrying one
support pair and two co-surface seam meridians with dihedral zero. The
surface is smooth through it, with no wedge and no ball-rest
configuration, so no run-out policy applies. A chain ending there refuses
`BlendError::UnsupportedCorner { corner: CornerConfig::SeamVertex,
policy: None }`; `CornerConfig::policy` is the single tag-to-policy map,
so a payload cannot disagree with its tag. `battery::is_seam_vertex`
reads pure incidence, never convexity, which fixes the rule: **a
recourse must be true at every site its tag can fire**.
`FILLET3_SEAM_VERTEX_RECOURSE` therefore names the REQUEST (ask for the
rim whole, every arc the seam split it into), and the closed-rim
surgery serves it on either material side: it takes that multi-link
closed chain as one annulus (`AnnulusRim::crossings`, one
`SeamCrossing` per arc, each side's support several faces of one
surface), removing material on a convex rim and adding it on a concave
one, so the sentence conditions on nothing. A pole-touching body with
merged caps (`merge_coplanar_faces` — the repair every boolean consumer
runs) hosts every arc on ONE plane face, in that face's own outer
cycle. The same annulus serves that too: `resolve_rim` routes it there
on WHERE the rim sits in its host's loop structure (a ring is the
ladder, the face's own outer cycle is this), and each crossing's host
foot is minted by the LADDER's strut (`HostFoot::Strut`) because the
merge consumed the host's seam and left the crossing TRIVALENT. The tag
does not fire at such a crossing — there is no seam there to make a
seam vertex — and the subset request that does refuse names the whole
rim, which carves. **Two conditions on that host, and they are what the
recourse states**: it carries no RING of its own, and the rim is its
WHOLE outer cycle. A merged cap that is an ANNULUS meets neither and
refuses (`work/fillet/hostless-rim-on-a-ringed-host-refuses.md`); a
CURVED single face carrying every arc is authorable through `topo`'s
`kef` and refuses at the half-band gate on both routes
(`work/fillet/curved-single-host-rim-refuses-at-the-half-band-gate.md`).

**A3-3 — the genuine mid-curve run-out is named and not implemented.**
Stopping a band part-way along a smooth rim, at a station with no
vertex, has two honest shapes: a ball-cap stop (the ball at rest at the
final spine station caps the band with a sphere patch; new surgery, no
new surface kinds) and a feather-out (the radius tapers to zero toward
the station; variable-radius machinery). Neither has a constructor
(`RunOutPolicy` is refusal-payload vocabulary only); the ball-cap is
the presumptive first pick when a consumer arrives.
