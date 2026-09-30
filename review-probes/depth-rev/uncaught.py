import shutil, sys, tempfile, traceback
from pathlib import Path

sys.path.insert(0, "/home/user/cad/.claude/worktrees/agent-af0c53382b4790f55/crates/pncad-py/tests")
import bench_scene
from pncad import DocRef, Doc, EvaluationError, Node, Workspace, content_pin, evaluate

levels = int(sys.argv[1])
mode = sys.argv[2]
directory = Path(tempfile.mkdtemp())
store = Workspace(str(directory))
below = bench_scene.post()
store.create(below)
for level in range(1, levels + 1):
    doc = Doc(f"pncad-depth-level-{level}")
    doc.insert(Node.instantiate_part(DocRef(below.id, content_pin(below))))
    store.create(doc)
    below = doc
top = Doc("pncad-depth-top")
instance = top.insert(Node.instantiate_part(DocRef(below.id, content_pin(below))))
ev = evaluate(top, resolver=store)
if mode == "format":
    try:
        ev.value(instance)
    except EvaluationError:
        text = traceback.format_exc()
        print("format_exc lines:", text.count("\n"), "has depth sentence:", "deeper than 1024" in text)
elif mode == "uncaught":
    ev.value(instance)
