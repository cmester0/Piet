import sys
from smpl_to_stk import *
from stk_interpreter import *
from stk_to_piet import *
from ascii_piet_to_piet import *
from piet_interpreter import *

smpl_name = sys.argv[1]
name = smpl_name[:smpl_name.rfind(".")]
stk_name = name + ".stk"
ascii_name = name + ".ascii"
piet_name = name + ".png"

smpl_to_stk(smpl_name, stk_name)
stk_to_piet(stk_name, ascii_name, optim=True)
ascii_piet_to_piet(ascii_name, piet_name)

stk_interpreter(stk_name)
piet_interpreter(piet_name)

# ascii_piet_to_piet(sys.argv[1])
