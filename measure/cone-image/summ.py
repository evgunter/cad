import sys,re,glob,os,collections
d=sys.argv[1]
for line in open(d+'/summary.txt'):
    if line.startswith('DONE'): continue
    test,res=line.split()
    name=test.replace(':','_')
    gate=[]
    for f in glob.glob(f'{d}/gl-{name}/*.log'):
        gate+= [l.rstrip('\n').split('\t') for l in open(f,errors='replace') if l.startswith('GATE')]
    verd=collections.Counter(re.sub(r'\dv\d|\d+v\d+','K',g[9])[:70] if 'Embedded'!=g[9] else 'Embedded' for g in gate)
    print(f'## {test} {res} asks={len(gate)}')
    for k,v in verd.most_common(): print('   ',v,k)
    log=open(f'{d}/{name}.log',errors='replace').read().split('\n')
    # image kinds over all cone faces: non-iso rows
    kinds=collections.Counter()
    for l in log:
        if l.startswith('CVI') and 'surface=Cone' in l:
            m=re.search(r'pcurve=(\w+) carrier=Some\("(\w+)"\)',l)
            if not m: continue
            iso = 'pcurve=Harmonic' in l and ('winding: 0' in l or re.search(r'v: Free \{ c: [^,]+, a: -?0\.0, b: -?0\.0, l: -?0\.0',l))
            kinds[(m.group(1),m.group(2),'iso' if iso else 'NON-ISO')]+=1
    print('    cone rows:',dict(kinds))
    cvs=collections.Counter()
    for l in log:
        if l.startswith('CVS'):
            cvs[' '.join(l.split()[1:3])]+=1
    print('    exits:',dict(cvs))
    cvt=collections.Counter()
    for l in log:
        if l.startswith('CVT') and 'Structural' in l:
            p=l.split(' | ')
            cvt[(p[-2][:16],p[-1][:16], 'ex=' in l and re.search(r'ex=\[([^\]]*)\]',l).group(1).count('EdgeKey'), re.search(r'nv=\[([^\]]*)\]',l).group(1).count('VertexKey'))]+=1
    print('    structural pairs (px,py,#near edges,#near verts):',dict(cvt))
