from lib import *
from i2 import arrowhead
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
def vdot(ic,x,y):
    """a point the user places while making the shape, punched out of whatever it sits on"""
    ic.dot(x,y,5.8,GAP); ic.dot(x,y,3.4,A3)
def on_circle(cx,cy,r,deg): a=math.radians(deg); return (cx+r*math.cos(a),cy+r*math.sin(a))
def guide(ic,a,b): ic.stroke(poly([a,b],False),P3,2.0)

@icon
def line():
    ic=Icon('line','Line'); ic.stroke('M15 49L49 15',A3,AW); vdot(ic,15,49); vdot(ic,49,15); return ic
@icon
def line_mid():
    ic=Icon('line_mid','Midpoint line'); ic.stroke('M12 50L52 14',A3,AW); vdot(ic,32,32); vdot(ic,52,14); return ic
@icon
def rect():
    ic=Icon('rect','Two-point rectangle'); d=poly([(10,16),(54,16),(54,48),(10,48)])
    ic.stroke(d,A3,AW,fill=A1); vdot(ic,10,16); vdot(ic,54,48); return ic
@icon
def rect3():
    ic=Icon('rect3','Three-point rectangle'); A,B=(8.2,36.3),(40.4,9.3); C,D=(55.8,27.7),(23.6,54.7)
    ic.stroke(poly([A,B,C,D]),A3,AW,fill=A1); vdot(ic,*A); vdot(ic,*B); vdot(ic,*C); return ic
@icon
def rect_center():
    ic=Icon('rect_center','Centre rectangle'); d=poly([(10,18),(54,18),(54,46),(10,46)])
    ic.stroke(d,A3,AW,fill=A1); vdot(ic,32,32); vdot(ic,54,18); return ic
@icon
def rect_rounded():
    ic=Icon('rect_rounded','Rounded rectangle'); d=rrect(10,16,54,48,9)
    ic.stroke(d,A3,AW,fill=A1); vdot(ic,10,16); vdot(ic,54,48); return ic
@icon
def circle():
    ic=Icon('circle','Centre circle'); ic.stroke(ell(32,32,22),A3,AW,fill=A1)
    vdot(ic,32,32); vdot(ic,*on_circle(32,32,22,-30)); return ic
@icon
def circle2():
    ic=Icon('circle2','Two-point circle'); ic.stroke(ell(32,32,21.6),A3,AW,fill=A1)
    vdot(ic,14,44); vdot(ic,50,20); return ic
@icon
def circle3():
    ic=Icon('circle3','Three-point circle'); ic.stroke(ell(32,32,22),A3,AW,fill=A1)
    for a in (-90,30,150): vdot(ic,*on_circle(32,32,22,a))
    return ic
@icon
def circle_tt():
    ic=Icon('circle_tt','Circle tangent to two lines')
    ic.stroke('M5 12L59 12M12 5L12 59',P3,AW)
    ic.stroke(ell(32,32,20),A3,AW,fill=A1); vdot(ic,32,12); vdot(ic,12,32); return ic
@icon
def circle_ttt():
    ic=Icon('circle_ttt','Circle tangent to three lines')
    ic.stroke(poly([(32,6),(5.5,51.5),(58.5,51.5)]),P3,AW)
    ic.stroke(ell(32,37,14.5),A3,AW,fill=A1)
    for p in ((32,51.5),(18.8,28.8),(45.2,28.8)): vdot(ic,*p)
    return ic
@icon
def arc3():
    ic=Icon('arc3','Three-point arc'); ic.stroke('M8 46A24 24 0 0 1 56 46',A3,AW)
    vdot(ic,8,46); vdot(ic,56,46); vdot(ic,32,22); return ic
@icon
def arc_center():
    ic=Icon('arc_center','Centre-point arc'); c=(32,44)
    s=on_circle(*c,24,-160); e=on_circle(*c,24,-20)
    guide(ic,c,s); guide(ic,c,e)
    ic.stroke('M%s %sA24 24 0 0 1 %s %s'%(f(s[0]),f(s[1]),f(e[0]),f(e[1])),A3,AW)
    vdot(ic,*c); vdot(ic,*s); vdot(ic,*e); return ic
@icon
def arc_tangent():
    ic=Icon('arc_tangent','Tangent arc'); ic.stroke('M6 52L24 52',P3,AW)
    ic.stroke('M24 52A26 26 0 0 0 50 26',A3,AW); vdot(ic,24,52); vdot(ic,50,26); return ic
@icon
def ellipse():
    ic=Icon('ellipse','Ellipse'); ic.stroke(ell(32,32,26,17),A3,AW,fill=A1)
    vdot(ic,32,32); vdot(ic,58,32); vdot(ic,32,15); return ic
@icon
def polygon():
    ic=Icon('polygon','Polygon'); ps=[on_circle(32,32,24,-90+60*i) for i in range(6)]
    ic.stroke(poly(ps),A3,AW,fill=A1); vdot(ic,32,32); vdot(ic,*ps[1]); return ic
@icon
def slot():
    ic=Icon('slot','Slot'); ic.stroke(rrect(7,21,57,43,11),A3,AW,fill=A1)
    vdot(ic,18,32); vdot(ic,46,32); vdot(ic,46,21); return ic
@icon
def arc_slot():
    ic=Icon('arc_slot','Arc slot'); cx,cy,R,hw=32,46,21,7; a0,a1=205,335
    def pe(r,deg): return on_circle(cx,cy,r,deg)
    o0,o1,i1,i0=pe(R+hw,a0),pe(R+hw,a1),pe(R-hw,a1),pe(R-hw,a0)
    d='M%s %s%s%s%s%sZ'%(f(o0[0]),f(o0[1]),arcto(R+hw,R+hw,o1[0],o1[1],0,1),arcto(hw,hw,i1[0],i1[1],0,1),arcto(R-hw,R-hw,i0[0],i0[1],0,0),arcto(hw,hw,o0[0],o0[1],0,1))
    ic.stroke(d,A3,AW,fill=A1); vdot(ic,*pe(R,a0)); vdot(ic,*pe(R,a1)); vdot(ic,*pe(R,270)); return ic
@icon
def spline():
    ic=Icon('spline','Fit-point spline'); ic.stroke('M9 44C17 14 27 14 32 30C37 46 47 46 55 20',A3,AW)
    for p in ((9,44),(21.6,19.75),(42.4,40.75),(55,20)): vdot(ic,*p)
    return ic
@icon
def spline_cv():
    ic=Icon('spline_cv','Control-point spline'); c=[(8,48),(20,10),(44,54),(56,16)]
    guide(ic,c[0],c[1]); guide(ic,c[1],c[2]); guide(ic,c[2],c[3])
    ic.stroke('M8 48C20 10 44 54 56 16',A3,AW)
    for p in c: vdot(ic,*p)
    return ic
@icon
def conic():
    ic=Icon('conic','Conic'); ic.stroke(arcpts(32,34,24,17,195,345),A3,AW)
    for t in (195,270,345): vdot(ic,32+24*math.cos(math.radians(t)),34+17*math.sin(math.radians(t)))
    return ic
@icon
def blend():
    ic=Icon('blend','Blend curve'); ic.stroke('M6 48L18 48M46 16L58 16',P3,AW)
    ic.stroke('M18 48C36 48 28 16 46 16',A3,AW); vdot(ic,18,48); vdot(ic,46,16); return ic
@icon
def centerline():
    ic=Icon('centerline','Centerline'); ic.stroke('M10 32L23 32M41 32L54 32',A3,AW); ic.dot(32,32,3.6,A3)
    vdot(ic,10,32); vdot(ic,54,32); return ic
