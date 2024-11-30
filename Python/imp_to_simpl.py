import sys

def handle_smpl_instr(var_list, instrs, index, l):
    next_index = index

    if len(l) == 1 and l[0] in matches:
        for smpl_instr in matches[l[0]]:
            index, next_index = handle_smpl_instr(var_list, instrs, index, smpl_instr)
        return index, next_index
