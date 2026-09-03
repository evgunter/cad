"""The mesh door: tessellate, read the mesh back, cross-check, export STL.

Steps 4 and 5 of the guide's ladder, from Python (LIB-G11). The two
helpers at the top are what makes step 5 worth running: the mesh-vs-exact
cross-check is only evidence if the second measure is computed
INDEPENDENTLY of the first, so these re-derive volume and closure from
the mesh's own arrays rather than asking the kernel a second time.

`mesh::validate`'s `check_mesh` / `signed_volume` / `triangle_count` do
the same job in Rust and are deliberately NOT bound: they are not on the
façade's curated lists (only `pncad::mesh::validate::*` names them), so
binding them would reach past the curation — and a caller-written
divergence-theorem sum is the more honest cross-check anyway.
"""

import math
import unittest

import pncad
from pncad import (
    BooleanOp,
    Doc,
    Node,
    Open,
    SketchPlane,
    Start,
    circle,
    deg,
    evaluate,
    m,
    mm,
)


def mesh_signed_volume(mesh):
    """The divergence-theorem volume of the mesh, in m³.

    Sum of the tetrahedra (o, a, b, c) over every triangle, with `o`
    the positions' bounding-box centre: for a closed mesh the anchor
    cancels out over the reals, and a body-scale one keeps the products
    from cancelling against the body's distance from the world origin.
    With the OUTWARD winding the patches promise this is positive for a
    closed body, and it uses nothing but `positions` and `triangles` —
    no kernel measure, which is the whole point.
    """
    p = [tuple(q.meters for q in point) for point in mesh.positions]
    if not p:
        return 0.0
    lo = [min(q[d] for q in p) for d in range(3)]
    hi = [max(q[d] for q in p) for d in range(3)]
    o = [lo[d] + (hi[d] - lo[d]) * 0.5 for d in range(3)]
    total = 0.0
    for i, j, k in mesh.triangles:
        (ax, ay, az), (bx, by, bz), (cx, cy, cz) = (
            tuple(q[d] - o[d] for d in range(3)) for q in (p[i], p[j], p[k])
        )
        total += (
            ax * (by * cz - bz * cy)
            - ay * (bx * cz - bz * cx)
            + az * (bx * cy - by * cx)
        )
    return total / 6.0


def unmatched_half_edges(mesh):
    """Directed triangle edges with no opposite twin.

    A closed 2-manifold's triangles pair up: every directed edge
    `(i, j)` is matched by exactly one `(j, i)`. Empty means watertight
    AND consistently oriented — and it is decided on INDICES, which is
    what the shared position buffer buys: two faces meeting along a
    boundary share the indices, so no coordinate comparison and no
    tolerance enters this check at all.
    """
    seen = {}
    for tri in mesh.triangles:
        for a, b in ((tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])):
            seen[(a, b)] = seen.get((a, b), 0) + 1
    return sorted(e for e, n in seen.items() if n != 1 or seen.get(e[::-1], 0) != 1)


def box(doc, width, depth, height):
    """An axis-aligned box at the origin, in metres."""
    outline = (
        Open.at((0 * m, 0 * m))
        .line_to((width * m, 0 * m))
        .line_to((width * m, depth * m))
        .line_to((0 * m, depth * m))
        .line_to(Start)
    )
    return doc.insert(Node.extrude(doc.insert(Node.profile(outline, plane=doc.sketch_frame())), height * m))


def body_of(doc, node):
    return evaluate(doc).value(node).body()


class TestTessellateIsADistance(unittest.TestCase):
    """δ crosses as a `Length` and is refused, never clamped."""

    def setUp(self):
        self.doc = Doc()
        self.body = body_of(self.doc, box(self.doc, 2.0, 3.0, 1.0))

    def test_the_budget_is_a_length_not_a_float(self):
        self.body.tessellate(1 * mm)
        with self.assertRaises(TypeError):
            self.body.tessellate(0.001)

    def test_a_nonpositive_budget_refuses_typed(self):
        for bad in (0 * mm, -1 * mm):
            with self.subTest(budget=bad.meters):
                with self.assertRaises(pncad.TessellateError) as caught:
                    self.body.tessellate(bad)
                self.assertEqual(caught.exception.variant, "invalid_chordal_tolerance")
                self.assertEqual(caught.exception.value, bad.meters)
                # The payload attributes are ALWAYS present, `None`
                # where the arm has nothing to put there, so a
                # stub-guided read cannot `AttributeError`.
                self.assertIsNone(caught.exception.bound)
                self.assertIsNone(caught.exception.requested)
                self.assertIsNone(caught.exception.note)

    def test_a_nonfinite_budget_refuses_typed(self):
        with self.assertRaises(pncad.TessellateError) as caught:
            self.body.tessellate(float("inf") * m)
        self.assertEqual(caught.exception.variant, "invalid_chordal_tolerance")

    def test_the_message_is_the_tessellators_prose_not_a_struct_dump(self):
        """The message is the door's own sentence, not `Debug`.

        `variant` is the branchable part; the message is for a human,
        so it has to READ as one — the tessellator's vocabulary, and
        none of the fingerprints a struct dump leaves behind."""
        with self.assertRaises(pncad.TessellateError) as caught:
            self.body.tessellate(0 * mm)
        message = str(caught.exception)
        self.assertIn("tessellate", message)
        self.assertIn("chordal tolerance", message)
        # A struct dump would carry the variant's identifier and the
        # brace-and-field punctuation; prose carries neither.
        self.assertNotIn("{", message)
        self.assertNotIn("InvalidChordalTolerance", message)

    def test_the_refusal_is_a_pncad_error(self):
        self.assertTrue(issubclass(pncad.TessellateError, pncad.PncadError))
        self.assertTrue(issubclass(pncad.StlError, pncad.PncadError))


class TestMeshReadBack(unittest.TestCase):
    """The mesh's arrays, and the two contracts they carry across."""

    def setUp(self):
        self.doc = Doc()
        self.body = body_of(self.doc, box(self.doc, 2.0, 3.0, 1.0))
        self.mesh = self.body.tessellate(1 * mm)

    def test_a_box_is_six_patches_and_twelve_triangles(self):
        self.assertEqual(self.mesh.patch_count, 6)
        self.assertEqual(self.mesh.triangle_count, 12)
        self.assertEqual(len(self.mesh.triangles), 12)
        self.assertEqual(len(self.mesh.positions), 8)

    def test_the_patches_concatenate_to_the_triangle_list(self):
        """`triangles` IS the patches in export order — the walk the
        STL writers make, so the two cannot disagree about a facet."""
        joined = []
        for i in range(self.mesh.patch_count):
            joined.extend(self.mesh.patch(i))
        self.assertEqual(joined, self.mesh.triangles)

    def test_a_patch_past_the_end_is_an_index_error(self):
        with self.assertRaises(IndexError):
            self.mesh.patch(self.mesh.patch_count)

    def test_every_index_points_into_the_shared_buffer(self):
        n = len(self.mesh.positions)
        for tri in self.mesh.triangles:
            for i in tri:
                self.assertLess(i, n)

    def test_positions_are_lengths(self):
        x, y, z = self.mesh.positions[0]
        for q in (x, y, z):
            self.assertIsInstance(q, pncad.Length)
        corners = {
            tuple(round(q.meters, 12) for q in p) for p in self.mesh.positions
        }
        self.assertIn((2.0, 3.0, 1.0), corners)

    def test_the_same_budget_gives_the_same_mesh(self):
        """D9: the kernel is deterministic, so two tessellations of one
        body at one budget agree bitwise — including the minting ORDER
        of the shared buffer, which the indices depend on."""
        again = self.body.tessellate(1 * mm)
        self.assertEqual(self.mesh.triangles, again.triangles)
        self.assertEqual(
            [tuple(q.meters for q in p) for p in self.mesh.positions],
            [tuple(q.meters for q in p) for p in again.positions],
        )


class TestWatertight(unittest.TestCase):
    """The closure contract, checked on indices."""

    def test_a_closed_body_has_no_unmatched_half_edge(self):
        doc = Doc()
        body = body_of(doc, box(doc, 2.0, 3.0, 1.0))
        self.assertEqual(unmatched_half_edges(body.tessellate(1 * mm)), [])

    def test_a_curved_body_is_watertight_too(self):
        """The cylinder's grid-sampled patches meet the planar caps'
        CDT along the same boundary indices — which is the property
        that makes a mesh watertight by construction rather than by a
        repair pass."""
        doc = Doc()
        disc = doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m), plane=doc.sketch_frame()))
        cyl = doc.insert(Node.extrude(disc, 2 * m))
        self.assertEqual(unmatched_half_edges(body_of(doc, cyl).tessellate(5 * mm)), [])

    def test_a_body_with_a_hole_is_watertight(self):
        doc = Doc()
        outer = (
            Open.at((-3 * m, -1.5 * m))
            .line_to((3 * m, -1.5 * m))
            .line_to((3 * m, 1.5 * m))
            .line_to((-3 * m, 1.5 * m))
            .line_to(Start)
        )
        hole = circle((0 * m, 0 * m), 0.7 * m)
        plate = doc.insert(
            Node.extrude(doc.insert(Node.profile([outer, hole], plane=doc.sketch_frame())), 0.6 * m)
        )
        self.assertEqual(unmatched_half_edges(body_of(doc, plate).tessellate(5 * mm)), [])


class TestBudget(unittest.TestCase):
    """δ is a DISPLAY parameter: it changes the mesh, never the body."""

    def test_a_finer_budget_buys_more_triangles_on_a_curve(self):
        doc = Doc()
        disc = doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m), plane=doc.sketch_frame()))
        body = body_of(doc, doc.insert(Node.extrude(disc, 2 * m)))
        coarse = body.tessellate(20 * mm)
        fine = body.tessellate(1 * mm)
        self.assertLess(coarse.triangle_count, fine.triangle_count)
        # Same body, both times: the exact measure does not move.
        self.assertEqual(body.mass_properties().volume, body.mass_properties().volume)

    def test_a_planar_body_is_the_same_mesh_at_any_budget(self):
        """A plane's chordal deviation is exactly zero, so there is
        nothing for a finer budget to buy."""
        doc = Doc()
        body = body_of(doc, box(doc, 2.0, 3.0, 1.0))
        self.assertEqual(
            body.tessellate(100 * mm).triangles,
            body.tessellate(0.001 * mm).triangles,
        )


class TestStlExport(unittest.TestCase):
    """The two writers, as doors that answer the bytes."""

    def setUp(self):
        doc = Doc()
        self.mesh = body_of(doc, box(doc, 2.0, 3.0, 1.0)).tessellate(1 * mm)

    def test_ascii_names_the_solid_and_carries_every_facet(self):
        text = self.mesh.to_stl_ascii(solid_name="brick")
        self.assertTrue(text.startswith("solid brick\n"))
        self.assertIn("endsolid brick", text)
        self.assertEqual(text.count("facet normal"), self.mesh.triangle_count)

    def test_binary_declares_the_facet_count_in_its_header(self):
        data = self.mesh.to_stl_binary(header="pncad, exported from Python")
        self.assertEqual(len(data), 84 + 50 * self.mesh.triangle_count)
        declared = int.from_bytes(data[80:84], "little")
        self.assertEqual(declared, self.mesh.triangle_count)
        self.assertTrue(data[:80].startswith(b"pncad, exported from Python"))

    def test_both_writers_have_a_default(self):
        self.assertTrue(self.mesh.to_stl_ascii().startswith("solid "))
        self.assertEqual(len(self.mesh.to_stl_binary()[:80]), 80)

    def test_an_unwritable_solid_name_refuses_at_the_call(self):
        """Validated, not sanitized: a newline would make
        `endsolid <name>` unmatchable and the file unparseable."""
        with self.assertRaises(pncad.StlError) as caught:
            self.mesh.to_stl_ascii(solid_name="two\nlines")
        self.assertEqual(caught.exception.variant, "solid_name_unrepresentable")

    def test_a_header_that_sniffs_as_ascii_refuses(self):
        with self.assertRaises(pncad.StlError) as caught:
            self.mesh.to_stl_binary(header=" Solid v2")
        self.assertEqual(caught.exception.variant, "binary_header_sniffs_ascii")

    def test_a_header_that_does_not_fit_refuses_rather_than_truncating(self):
        with self.assertRaises(pncad.StlError) as caught:
            self.mesh.to_stl_binary(header="x" * 81)
        self.assertEqual(caught.exception.variant, "binary_header_too_long")


class TestCrossCheckOnBooleanGeometry(unittest.TestCase):
    """Step 5 on a body a boolean built, where the mesh and the exact
    measure have the least in common."""

    def test_a_subtracted_pocket_measures_the_same_both_ways(self):
        doc = Doc()
        base = box(doc, 4.0, 4.0, 1.0)
        tool_p = doc.insert(
            Node.polygon(
                [
                    (1 * m, 1 * m),
                    (3 * m, 1 * m),
                    (3 * m, 3 * m),
                    (1 * m, 3 * m),
                ],
                plane=doc.sketch_frame(elevation=0.5 * m),
            )
        )
        tool = doc.insert(Node.extrude(tool_p, 1 * m))
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, base, tool))
        body = body_of(doc, cut)
        body.validate()

        exact = body.mass_properties().volume
        self.assertAlmostEqual(exact, 4.0 * 4.0 * 1.0 - 2.0 * 2.0 * 0.5, delta=1e-12)

        mesh = body.tessellate(1 * mm)
        self.assertEqual(unmatched_half_edges(mesh), [])
        self.assertLess(abs(mesh_signed_volume(mesh) - exact) / exact, 1e-12)


class TestCrossCheckOnASketchPlane(unittest.TestCase):
    """The same check on a body whose faces are not axis-aligned —
    a prism on a general `from_frame` plane, where a winding mistake
    would not cancel."""

    def test_a_tilted_prism_measures_the_same_both_ways(self):
        s = math.sqrt(0.5)
        plane = SketchPlane.from_frame(
            (0 * m, 0 * m, 0 * m), (s, s, 0.0), (0.0, 0.0, 1.0)
        )
        doc = Doc()
        sketch = doc.insert(
            Node.polygon(
                [(0 * m, 0 * m), (2 * m, 0 * m), (2 * m, 1 * m), (0 * m, 1 * m)],
                plane=doc.sketch_frame(plane=plane),
            )
        )
        prism = doc.insert(Node.extrude(sketch, 3 * m))
        body = body_of(doc, prism)
        body.validate()

        exact = body.mass_properties().volume
        mesh = body.tessellate(1 * mm)
        self.assertEqual(unmatched_half_edges(mesh), [])
        self.assertGreater(mesh_signed_volume(mesh), 0.0)
        self.assertLess(abs(mesh_signed_volume(mesh) - exact) / exact, 1e-12)


class TestCrossCheckConverges(unittest.TestCase):
    """On a curved body the two measures differ BY THE BUDGET, and the
    difference shrinks with it — which is the statement the chordal
    certificate makes, seen from outside."""

    def test_the_error_shrinks_with_the_budget(self):
        doc = Doc()
        outline = (
            Open.at((0.5 * m, 0 * m))
            .line_to((1.5 * m, 0 * m))
            .line_to((1.5 * m, 2 * m))
            .line_to((0.5 * m, 2 * m))
            .line_to(Start)
        )
        frame = doc.sketch_frame()
        # The axis in the sketch's own coordinates: the frame's v is
        # world +y, so the world y axis IS its own +y through (0, 0).
        axis = doc.insert(Node.datum_axis_in_plane(frame, (0 * m, 0 * m), (0.0, 1.0)))
        ring = doc.insert(
            Node.revolve(doc.insert(Node.profile(outline, plane=frame)), axis, 360 * deg)
        )
        body = body_of(doc, ring)
        body.validate()
        exact = body.mass_properties().volume
        self.assertAlmostEqual(exact, math.pi * (1.5**2 - 0.5**2) * 2.0, delta=1e-9)

        errors = []
        for budget in (20 * mm, 5 * mm, 1 * mm):
            mesh = body.tessellate(budget)
            self.assertEqual(unmatched_half_edges(mesh), [])
            errors.append(abs(mesh_signed_volume(mesh) - exact) / exact)
        # The chordal certificate is a bound on a SAGITTA, so the
        # volume error is first order in δ: quartering the budget
        # should buy roughly a quarter of the error. Measured
        # 6.2e-3 -> 1.6e-3 -> 3.3e-4 (ratios 3.8 and 4.9); the pin is
        # a factor of two, well clear of both, because what is being
        # asserted is CONVERGENCE and not a rate.
        self.assertGreater(errors[0], 2 * errors[1])
        self.assertGreater(errors[1], 2 * errors[2])
        self.assertLess(errors[-1], 1e-3)


if __name__ == "__main__":
    unittest.main()
