"""place2-r2: one A∘F∘B row through Python, against an independent composition."""
import math, shutil, sys, tempfile
sys.path.insert(0, "/home/user/place2-r2/crates/pncad-py/tests")
import pncad
from pncad import CapEnd, ContactClass, Doc, DocEdit, Frame, Node, Placement, Workspace, m, solve_document, evaluate
import bench_scene

def mat(f):
    c = f.columns  # columns
    r = [[c[j][i] for j in range(3)] for i in range(3)]
    t = [x.meters for x in f.origin]
    return r, t

def comp(a, b):
    (ra, ta), (rb, tb) = a, b
    r = [[sum(ra[i][k] * rb[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    t = [sum(ra[i][k] * tb[k] for k in range(3)) + ta[i] for i in range(3)]
    return r, t

def dist(a, b):
    return max(max(abs(a[0][i][j] - b[0][i][j]) for i in range(3) for j in range(3)),
               max(abs(a[1][i] - b[1][i]) for i in range(3)))

d = tempfile.mkdtemp()
try:
    ws = Workspace(d)
    (post_doc, post_ref), (_, shelf_ref) = bench_scene.parts(ws)
    post_cap = bench_scene.part_cap(post_doc, CapEnd.End)
    doc = Doc("r2-py-afb")
    g0f = Frame.translation((1 * m, 0 * m, 0 * m))
    g1f = Frame.rotate_then_translate((0.0, 0.0, 1.0), 0.6 * pncad.rad, (3 * m, 4 * m, 5 * m))
    of = Frame.rotate_then_translate((1.0, 0.0, 0.0), 0.2 * pncad.rad, (0 * m, 0 * m, 1 * m))
    g0 = doc.insert(Node.gauge(Placement.literal(g0f)))
    g1 = doc.insert(Node.gauge(Placement.literal(g1f), g0))
    post = doc.insert(Node.instantiate_part(post_ref))
    doc.apply(DocEdit.set_gauge(post, g1))
    doc.apply(DocEdit.set_offset(post, Placement.literal(of)))
    shelf = doc.insert(Node.instantiate_part(shelf_ref))
    doc.apply(DocEdit.set_gauge(shelf, g1))
    mate = Node.mate(
        shelf, bench_scene.instance_face(ws, doc, shelf, CapEnd.Start),
        post, bench_scene.instance_face(ws, doc, post, CapEnd.End),
        ContactClass.Rest, bench_scene.seat(*bench_scene.STAND_SEATS[0], post_cap=post_cap))
    doc.insert(mate, resolver=ws)
    poses = solve_document(doc, resolver=ws)
    rel = poses.relative(shelf)
    want = comp(comp(comp(mat(g0f), mat(g1f)), mat(of)), mat(rel))
    got = mat(poses.placement(doc, shelf))
    print("shelf world vs G0∘G1∘o∘relative:", dist(got, want))
    assert dist(got, want) <= 1e-12, (got, want)
    want_post = comp(comp(mat(g0f), mat(g1f)), mat(of))
    print("post world vs G0∘G1∘o:", dist(mat(poses.placement(doc, post)), want_post))
    assert dist(mat(poses.placement(doc, post)), want_post) <= 1e-12
    print("OK")
finally:
    shutil.rmtree(d, True)
