"""place2-r2: Python's assemble vs an unplaced group's own space."""
import shutil, sys, tempfile
sys.path.insert(0, "/home/user/place2-r2/crates/pncad-py/tests")
import pncad
from pncad import CapEnd, ContactClass, Doc, DocEdit, Frame, Node, Placement, Workspace, m, evaluate
import bench_scene

d = tempfile.mkdtemp()
try:
    ws = Workspace(d)
    (post_doc, post_ref), (_, shelf_ref) = bench_scene.parts(ws)
    post_cap = bench_scene.part_cap(post_doc, CapEnd.End)
    doc = Doc("r2-py-gate")
    p1 = doc.insert(Node.instantiate_part(post_ref))
    shelf = doc.insert(Node.instantiate_part(shelf_ref))
    p2 = doc.insert(Node.instantiate_part(post_ref))
    f = lambda n, s: bench_scene.instance_face(ws, doc, n, s)
    # shelf onto p1 (seat A), then p2 onto the shelf at the SAME seat: p2 == p1.
    doc.insert(Node.mate(shelf, f(shelf, CapEnd.Start), p1, f(p1, CapEnd.End), ContactClass.Rest,
                         bench_scene.seat(bench_scene.SEAT_A, bench_scene.POST_CAP, post_cap=post_cap)), resolver=ws)
    doc.insert(Node.mate(p2, f(p2, CapEnd.End), shelf, f(shelf, CapEnd.Start), ContactClass.Rest,
                         bench_scene.seat(bench_scene.POST_CAP, bench_scene.SEAT_A, post_cap=post_cap)), resolver=ws)
    lone = doc.insert(Node.instantiate_part(post_ref))
    doc.apply(DocEdit.set_offset(lone, Placement.literal(Frame.translation((1 * m, 0 * m, 0 * m)))))
    ev = evaluate(doc, resolver=ws)
    try:
        pncad.assemble(doc, ev)
        print("placed: python assemble certifies")
    except Exception as e:
        print("placed: python assemble refuses:", str(e)[:90])
    g = doc.insert(Node.gauge(Placement.identity()))
    for n in (p1, shelf, p2):
        doc.apply(DocEdit.set_gauge(n, g))
    doc.apply(DocEdit.delete_node(g))
    ev = evaluate(doc, resolver=ws)
    print("unplaced:", ev.unplaced if hasattr(ev, "unplaced") else "?")
    try:
        a = pncad.assemble(doc, ev)
        print("DEFECT: python assemble certifies with an interpenetrating own-space group; minted", len(a.minted))
    except Exception as e:
        print("unplaced: python assemble refuses:", str(e)[:120])
finally:
    shutil.rmtree(d, True)
