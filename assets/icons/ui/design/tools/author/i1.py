from lib import *
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn

def blue_cube(ic,I,x0,y0,z0,x1,y1,z1): I.box(ic,x0,y0,z0,x1,y1,z1,(A1,A2,A3))

@icon
def box():
    ic=Icon('box','Box'); I=Iso(1,32,32); s=27
    I.box(ic,0,0,0,s,s,s,(A1,A2,A3)); 
    # offset so centre lands on (32,32)
    return ic
@icon
def cylinder_():
    ic=Icon('cylinder','Cylinder'); cylinder(ic,32,15,20,34,(A1,A2,A3)); return ic
@icon
def sphere():
    ic=Icon('sphere','Sphere')
    ic.fill(ell(32,32,26),A1)
    p0=arc_pt=(32+26*math.cos(math.radians(-35)),32+26*math.sin(math.radians(-35)))
    p1=(32+26*math.cos(math.radians(125)),32+26*math.sin(math.radians(125)))
    ic.fill('M%s %sA26 26 0 0 1 %s %sA34 34 0 0 0 %s %sZ'%(f(p0[0]),f(p0[1]),f(p1[0]),f(p1[1]),f(p0[0]),f(p0[1])),A2)
    q0=(32+26*math.cos(math.radians(10)),32+26*math.sin(math.radians(10)))
    q1=(32+26*math.cos(math.radians(100)),32+26*math.sin(math.radians(100)))
    ic.fill('M%s %sA26 26 0 0 1 %s %sA30 30 0 0 0 %s %sZ'%(f(q0[0]),f(q0[1]),f(q1[0]),f(q1[1]),f(q0[0]),f(q0[1])),A3)
    ic.stroke(ell(32,32,26),INK,OW)
    ic.stroke(arcpts(32,32,26,8,8,172),INK,OW)
    return ic
@icon
def torus():
    ic=Icon('torus','Torus')
    outer=ell(32,32,26,17); inner=ell(32,30,11,5.5)
    ic.fill(outer,A2); ic.fill(ell(32,27,24,14),A1) if False else None
    ic.fill('M32 15A26 17 0 0 0 6 32A26 17 0 0 0 32 49A26 17 0 0 0 58 32A26 17 0 0 0 32 15Z',A2)
    # lit upper outer rim
    ic.fill('M7.5 27A26 17 0 0 1 56.5 27A24 14.5 0 0 0 7.5 27Z',A1)
    ic.stroke(outer,INK,OW)
    ic.fill(inner,GAP); ic.stroke(inner,INK,OW)
    return ic
