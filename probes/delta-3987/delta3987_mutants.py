import subprocess, sys, os, re
ROOT='/home/user/d3987'
OPS='crates/topo/src/boolean/ops.rs'; RED='crates/topo/src/boolean/reduce.rs'; MOD='crates/topo/src/boolean/mod.rs'; REST='crates/topo/src/boolean/rest.rs'
SORT_ONLY='''
fn gate_sort_only<T: Decide + Bounds + AtRestPolicy>(
    body: Body<T>,
    band: Band,
    tol: Tol,
) -> Result<AtRestBody<T>, BooleanError> {
    let mut body = body;
    let pad = super::boxes::sweep_pad(band);
    let face_box = |body: &Body<T>, f| super::boxes::face_box(body, f, pad, band).ok();
    crate::pieces::sort_into_pieces(&mut body, band, tol, T::quad_lane(), Some(&face_box))
        .map_err(BooleanError::Pieces)?;
    Ok(AtRestBody::not_run(body))
}
'''
M = {
 'MA_no_dual_orientation': [(RED, "crate::validate::inside_out_solids(body, band, tol, T::quad_lane()).first()",
                             "crate::validate::inside_out_solids(body, band, tol, T::quad_lane()).first().filter(|_| false)")],
 'MB_no_structural_gate': [(OPS, "        structural_gate(&kept)?;\n", "        if false { structural_gate(&kept)?; }\n")],
 'MC_reduce_takes_body': [
    (MOD, "pub fn boolean_reduce<T: Decide + Bounds + crate::props::AtRestPolicy>(\n    op: BooleanOp,\n    a_operand: &crate::AtRestBody<T>,\n    b_operand: &crate::AtRestBody<T>,",
          "pub fn boolean_reduce<T: Decide + Bounds + crate::props::AtRestPolicy>(\n    op: BooleanOp,\n    a_operand: &Body<T>,\n    b_operand: &Body<T>,"),
    (MOD, "    a_operand: &crate::AtRestBody<T>,\n    b_operand: &crate::AtRestBody<T>,\n    decls: &BooleanDeclarations,\n    tol: Tol,\n) -> Result<BooleanReduction<T>, BooleanError> {\n    let band = Band::linear(tol)?;\n    for (operand, body) in [(Operand::A, a_operand), (Operand::B, b_operand)] {\n        reduce::gate_unverdicted_operand(body, operand, band, tol)?;\n    }\n",
          "    a_operand: &Body<T>,\n    b_operand: &Body<T>,\n    decls: &BooleanDeclarations,\n    tol: Tol,\n) -> Result<BooleanReduction<T>, BooleanError> {\n"),
 ],
 'M3p_sort_only_at_fallback_sites': [
    (OPS, "            let body = gate(body, band, tol)?;\n            let (graft_vertices", "            let body = gate_sort_only(body, band, tol)?;\n            let (graft_vertices"),
    (OPS, "    remap_carried(&mut contacts, &body, decls, &a_view, &b_view, &desc)?;\n    let body = gate(body, band, tol)?;", "    remap_carried(&mut contacts, &body, decls, &a_view, &b_view, &desc)?;\n    let body = gate_sort_only(body, band, tol)?;"),
    (OPS, "/// The result gate where no at-rest gate ran", SORT_ONLY + "\n/// The result gate where no at-rest gate ran"),
    (REST, "    let body = gate(zipped, band, tol)?;", "    let body = super::ops::gate_sort_only(zipped, band, tol)?;"),
    (OPS, "\nfn gate_sort_only<", "\npub(super) fn gate_sort_only<"),
 ],
 'M2_mains_gate': [(OPS, "    #[cfg(feature = \"door-tier3-meter\")]\n    let from = std::time::Instant::now();\n    let kept = T::gate_at_rest_kept(body, tol);",
                    "    if true { structural_gate(&body)?; return Ok(AtRestBody::not_run(body)); }\n    #[cfg(feature = \"door-tier3-meter\")]\n    let from = std::time::Instant::now();\n    let kept = T::gate_at_rest_kept(body, tol);")],
}
name = sys.argv[1]
orig = {}
try:
    for f, a, b in M[name]:
        p = os.path.join(ROOT, f)
        s = open(p).read()
        orig.setdefault(p, s)
        assert s.count(a) == 1, (name, f, a[:60], s.count(a))
        open(p, 'w').write(s.replace(a, b))
    env = dict(os.environ, CARGO_TARGET_DIR='/home/user/d3987-target', CAD_TOLERANCE_EPS=os.environ.get('CAD_TOLERANCE_EPS', '1e-9'))
    pk = sys.argv[2:] or ['-p', 'topo', '-p', 'sweep', '-p', 'editor-core', '-p', 'verbs']
    log = f'/home/user/d3987-mut-{name}.log'
    with open(log, 'w') as fh:
        subprocess.run(['cargo', 'nextest', 'run', '--profile', 'ci', *pk, '--no-fail-fast', '-E', 'not test(r2_rotated_boxes)'], cwd=ROOT, env=env, stdout=fh, stderr=subprocess.STDOUT)
    out = open(log).read()
    errs = [l for l in out.splitlines() if l.startswith('error')]
    fails = sorted(set(re.findall(r'^\s+FAIL \[.*?\] \(\s*\d+/\d+\) (.*)$', out, re.M)))
    summ = re.findall(r'Summary.*', out)
    print(name, 'compile errors:', errs[:5])
    print(name, summ)
    for f in fails: print('  RED', f)
finally:
    for p, s in orig.items(): open(p, 'w').write(s)
