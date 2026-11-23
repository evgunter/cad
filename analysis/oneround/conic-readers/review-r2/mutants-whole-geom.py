import subprocess, sys, os, re
sys.path.insert(0,'/home/user/rv/mut')
src=open('/home/user/rv/mut/run.py').read().split('which = sys.argv')[0]
exec(src)
for w in sys.argv[1:]:
    name=[n for n in M if n.startswith(w+' ')][0]
    f,old,new=M[name]; path=R+f; orig=open(path).read()
    open(path,'w').write(orig.replace(old,new))
    try:
        env=dict(os.environ, CARGO_TARGET_DIR='/home/user/tgt-head')
        r=subprocess.run(['cargo','nextest','run','-p','geom','--no-fail-fast','-E','not test(review_)'],cwd=R,env=env,capture_output=True,text=True,timeout=1500)
        out=r.stdout+r.stderr
        summ=[l for l in out.splitlines() if 'Summary' in l or l.strip().startswith('FAIL [')]
        print(name, '\n   '+'\n   '.join(sorted(set(s.strip() for s in summ))[:12]), flush=True)
    finally:
        open(path,'w').write(orig)
