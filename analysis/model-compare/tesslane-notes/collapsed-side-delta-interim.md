DELTA INTERIM — PR 7517 @ 390036911f. No new MAJOR so far. Item (c) is DEMONSTRATED.

(c) Two different MESHES of one collapsed-side face at one δ, chosen by ε.
- Body: the wedge (square lofted with a hinge 0.5, v-degree 1), every vertex point restated +off via `topo::test_support::restate_geometry(Stored::Point => x + off)`. Meshed with `mesh::tessellate`, run once per ε (CAD_TOLERANCE_EPS).
- off = 1e-8, δ = 0.1: ε 1e-6 → 248 tris (side walked), digest 25e3945334089fe8; ε 1e-9 and ε 1e-12 → 240 tris (one copy), digest 53228aa5bd157cc1. All three closed (check_mesh ok).
- Same split at δ = 3e-2: 656 tris against 648.
- off = 1e-7: the same split.
- off = 1e-10 and 1e-11: ε 1e-12 → 240 tris, while ε 1e-9 and 1e-6 → 248.
- off = 1e-13 and 1e-5: one digest at every ε.
- Where the band is undecided (off 1e-9 at ε 1e-9, off 1e-6 at ε 1e-6), the face refuses `SphereTrimShape`, typed. No panic, and no DomainUnread or other refusal from the lift, anywhere in the sweep. So I found no case where topo's lift and the mesh decide differently; the two reads agree as built.
- Against main: on main the spline arm read no side at all, so this input meshed with one copy at every ε, the same bytes across ε. This PR makes it ε-dependent, a fresh instance of the pole-identification debt, which the row's new "second reader" section names abstractly.
- My call: NOT blocking. No door builds such a vertex: the loft writes the pinned corner as the side's own point, and `the_class_meshes_to_one_digest_at_every_eps` holds the class. [tolerance.chordal] accepts the read with the guard owed. Owed: pin this witness on `pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face.md`, and reconsider that row's P3, since it now has two readers and a concrete spline witness.

Also verified, by run:
- `bash scripts/run-gates.sh`: rc 0.
- `rg 'bit_identity|one_stored_point|eq_bits' crates/mesh`: empty.
- Delta head builds.

Running now: whole `mesh` with the slow set at the three ε. The full report follows.
