import sys
m = sys.argv[1]
p = '/tmp/claude-0/probe-wt/crates/mesh/src/planar.rs'
t = '/tmp/claude-0/probe-wt/crates/mesh/src/trimmed.rs'
def sub(path, a, b):
    s = open(path).read()
    assert s.count(a) == 1, (m, a)
    open(path, 'w').write(s.replace(a, b))
if m == 'first':
    sub(p, "        let refuse = TessellateError::PinchWedge { face: fk };\n",
           "        let refuse = TessellateError::PinchWedge { face: fk };\n        if true { return Ok(Some(wedges[0].0)); }\n")
elif m == 'other':
    sub(p, "            sides == sector\n        });", "            sides != sector\n        });")
elif m == 'norefuse':
    sub(p, "            (Some(&(id, _)), None) => Ok(Some(id)),\n            _ => Err(refuse),",
           "            (Some(&(id, _)), None) => Ok(Some(id)),\n            _ => Ok(None),")
elif m == 'nobound':
    sub(p, "        let (Some(cw), Some(ccw)) = (bound(first, false), bound(first.ccw(), true)) else {\n            return Err(refuse);",
           "        let (Some(cw), Some(ccw)) = (bound(first, false), bound(first.ccw(), true)) else {\n            return Ok(None);")
elif m == 'twice':
    sub(p, "                if wedges.iter().any(|w| w.0 == id) {\n                    return Err(TessellateError::PinchWedge { face: fk });\n                }",
           "                if wedges.iter().any(|w| w.0 == id) {\n                    continue;\n                }")
elif m == 'trimmed_off':
    sub(t, "                    Ok(Some(id)) => ids[k] = PatchVertex::Shared(id),", "                    Ok(Some(_)) => {}")
elif m == 'last':
    sub(p, "        let refuse = TessellateError::PinchWedge { face: fk };\n",
           "        let refuse = TessellateError::PinchWedge { face: fk };\n        if true { return Ok(Some(wedges[wedges.len() - 1].0)); }\n")
