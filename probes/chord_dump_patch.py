import sys,re
path, kind = sys.argv[1], sys.argv[2]
s=open(path).read()
assert s.count("\nfn chord_spec<T: Decide>(")==1
s=s.replace("\nfn chord_spec<T: Decide>(","\nfn chord_spec_inner<T: Decide>(")
if kind=="head":
    params="body: &mut Body<T>, band: Band, lane: JoinLane<'_, T>, face: FaceKey, u1: VertexKey, u2: VertexKey, leave: Vec3<T>"
    call="chord_spec_inner(body, band, lane, face, u1, u2, leave)"
else:
    params="body: &mut Body<T>, band: Band, lane: JoinLane<'_, T>, face: FaceKey, run: ChordRun<'_>, u1: VertexKey, u2: VertexKey"
    call="chord_spec_inner(body, band, lane, face, run, u1, u2)"
wrap=f'''
#[allow(clippy::too_many_arguments)]
fn chord_spec<T: Decide>({params}) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {{
    let pts = (vertex_point(body, u1).ok(), vertex_point(body, u2).ok());
    let r = {call};
    if let Ok(path) = std::env::var("CHORD_DUMP") {{
        use std::io::Write;
        let name = std::thread::current().name().unwrap_or("?").to_string();
        let what = match &r {{
            Ok(Some(spec)) => format!("SPEC {{:?}} {{:?}} {{:?}} {{:?}}", spec.carrier, spec.param_start, spec.param_end, pts),
            Ok(None) => format!("NONE {{:?}}", pts),
            Err(e) => format!("ERR {{:?}}", e),
        }};
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {{
            let _ = writeln!(f, "{{}}\\t{{}}\\t{{}}", name, std::process::id(), what.replace('\\n', " "));
        }}
    }}
    r
}}
'''
i=s.index("\nfn chord_spec_inner<T: Decide>(")
# insert before the doc comment block preceding: just insert before the fn's attribute lines is hard; append at end of file instead
s=s+wrap
open(path,"w").write(s)
