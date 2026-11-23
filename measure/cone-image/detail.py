import sys,re,glob,collections
d=sys.argv[1]
for line in open(d+'/summary.txt'):
    if line.startswith('DONE'): continue
    test,res=line.split(); name=test.replace(':','_')
    gate=[]
    for f in glob.glob(f'{d}/gl-{name}/*.log'):
        gate+= [l.rstrip('\n').split('\t') for l in open(f,errors='replace') if l.startswith('GATE')]
    log=open(f'{d}/{name}.log',errors='replace').read().split('\n')
    print('##',test)
    seen=set()
    for g in gate:
        if g[9]=='Embedded': continue
        m=re.search(r'a: FaceKey\((\w+)\), b: FaceKey\((\w+)\)(?:, shared: (\w+)\(\w+\((\w+)\)\))?',g[9])
        if not m: print('   ?',g[9][:120]); continue
        a,b,kind,key=m.groups()
        head=g[9].split('{')[0]+(g[9].split('stop: ')[1].split(' ')[0] if 'stop:' in g[9] else '')
        def rows(face,edge=None):
            out={}
            for l in log:
                if l.startswith('CVI') and f'face=FaceKey({face})' in l and (edge is None or f'edge=Some(EdgeKey({edge}))' in l):
                    mm=re.search(r'surface=(\w+) edge=Some\(EdgeKey\((\w+)\)\) pcurve=(\w+) carrier=Some\("(\w+)"\)',l)
                    if mm: out[mm.group(2)]=(mm.group(1),mm.group(3),mm.group(4))
            return out
        ra,rb=rows(a),rows(b)
        sa=next(iter(ra.values()))[0] if ra else '?'; sb=next(iter(rb.values()))[0] if rb else '?'
        sh = (ra.get(key),rb.get(key)) if kind=='Edge' else None
        sig=(head,sa,sb,kind,str(sh))
        if sig in seen: continue
        seen.add(sig)
        print(f'   {head.strip()} {sa}/{sb} shared={kind} images={sh} SITE={g[-1].split("SITE")[-1].strip()}')
        cvs=collections.Counter()
        for l in log:
            if l.startswith('CVS') and (f'face=FaceKey({a})' in l or f'face=FaceKey({b})' in l):
                cvs[' '.join(l.split()[1:3])]+=1
        print('      exits on these faces:',dict(cvs))
