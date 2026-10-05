import sys,subprocess,os
name,path,old,new=sys.argv[1:5]
filt=sys.argv[5] if len(sys.argv)>5 else 'test(/intent_literals_a|intent_vars|doc::tests|persist::check/)'
os.chdir('/home/user/cad')
s=open(path).read()
assert s.count(old)==1,(name,s.count(old))
open(path,'w').write(s.replace(old,new))
try:
    env=dict(os.environ,CARGO_TARGET_DIR='/home/user/r2target')
    r=subprocess.run(['cargo','nextest','run','-p','editor-core','--profile','default','--no-fail-fast','-E',filt],capture_output=True,text=True,env=env)
    out=r.stdout+r.stderr
    fails=[l.strip() for l in out.splitlines() if l.strip().startswith('FAIL [')]
    summ=[l for l in out.splitlines() if 'Summary' in l]
    errs=[l for l in out.splitlines() if l.startswith('error')][:3]
    print(name, 'KILLED' if fails else ('BUILD-ERR' if errs and not summ else 'SURVIVED'), summ[-1:] , sorted(set(fails))[:6], errs)
finally:
    subprocess.run(['git','checkout','--',path])
