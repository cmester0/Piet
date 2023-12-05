import sys
from smpl_to_stk import *
from stk_interpreter import *
from stk_to_ascii_piet import *
from ascii_piet_to_piet import *
from piet_interpreter import *

if len(sys.argv) <= 2:
    print ("missing argument, add one or more of",["gen","stk","piet","debug"],"seperated by a ','")

smpl_name = sys.argv[1]
name = smpl_name[:smpl_name.rfind(".")]
stk_name = name + ".stk"
ascii_name = name + ".ascii"
piet_name = name + ".png"

if "gen" in sys.argv[2]:
    smpl_to_stk(smpl_name, stk_name)
    stk_to_ascii_piet(stk_name, ascii_name, optim=True)
    ascii_piet_to_piet(ascii_name, piet_name)

if "stk" in sys.argv[2]:
    stk_interpreter(stk_name, "debug" in sys.argv[2])
if "piet" in sys.argv[2]:
    piet_interpreter(piet_name, debug="debug" in sys.argv[2])
