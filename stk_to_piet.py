import sys
import png

from generate_number import * # optimize number

def stk_to_piet(i_file, o_file, optim=True):
    # ⚪

    # ❤🧡💛💚💙💜
    # 🔴🟠🟡🟢🔵🟣
    # 🟥🟧🟨🟩🟦🟪

    # ⚫

    colors = [["❤","🧡","💛","💚","💙","💜"],
              ["🔴","🟠","🟡","🟢","🔵","🟣"],
              ["🟥","🟧","🟨","🟩","🟦","🟪"]]

    rev_map = {
        "❤": (0,0),
        "🔴": (0,1),
        "🟥": (0,2),
        "🧡": (1,0),
        "🟠": (1,1),
        "🟧": (1,2),
        "💛": (2,0),
        "🟡": (2,1),
        "🟨": (2,2),
        "💚": (3,0),
        "🟢": (3,1),
        "🟩": (3,2),
        "💙": (4,0),
        "🔵": (4,1),
        "🟦": (4,2),
        "💜": (5,0),
        "🟣": (5,1),
        "🟪": (5,2)}

    stack_optimizer = StackOptimizer()

    def index_and_command_to_color_and_next_index(cmd, output):
        previous_c, previous_r = rev_map[output[-1]]
        match cmd[0]:
            case "push":
                for _ in range(int(cmd[1])-1):
                    output.append(colors[previous_r][previous_c])
                previous_r = (previous_r + 1) % 3
            case "pop":
                previous_r = (previous_r - 1) % 3
            case "add":
                previous_c = (previous_c + 1) % 6
            case "sub":
                previous_c = (previous_c + 1) % 6
                previous_r = (previous_r + 1) % 3
            case "mul":
                previous_c = (previous_c + 1) % 6
                previous_r = (previous_r - 1) % 3
            case "div":
                previous_c = (previous_c + 2) % 6
            case "mod":
                previous_c = (previous_c + 2) % 6
                previous_r = (previous_r + 1) % 3
            case "not":
                previous_c = (previous_c + 2) % 6
                previous_r = (previous_r - 1) % 3
            case "greater":
                previous_c = (previous_c + 3) % 6
            case "pointer":
                previous_c = (previous_c + 3) % 6
                previous_r = (previous_r + 1) % 3
            case "switch":
                previous_c = (previous_c + 3) % 6
                previous_r = (previous_r - 1) % 3
            case "dup":
                previous_c = (previous_c + 4) % 6
            case "roll":
                previous_c = (previous_c + 4) % 6
                previous_r = (previous_r + 1) % 3
            case "inN":
                previous_c = (previous_c + 4) % 6
                previous_r = (previous_r - 1) % 3
            case "inC":
                previous_c = (previous_c + 5) % 6
            case "outN":
                previous_c = (previous_c + 5) % 6
                previous_r = (previous_r + 1) % 3
            case "outC":
                previous_c = (previous_c + 5) % 6
                previous_r = (previous_r - 1) % 3
            case default:
                print("Invalid cmd:", cmd)
        output.append(colors[previous_r][previous_c])

    def make_block(block):
        output = ["🔴"]
        for inp in block:
            cmd = inp.split()
            match cmd[0]:
                case "branch":
                    # pointer
                    previous_c, previous_r = rev_map[output[-1]]
                    previous_c = (previous_c + 3) % 6
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])
                    return output, [cmd[1], cmd[2]]
                case "goto":
                    return output, [cmd[1]]
                case "term":
                    return output, ["term"]
                case default:
                    index_and_command_to_color_and_next_index(cmd, output)
        return output, []

    inp_lines = []
    with open(i_file, 'r') as f:
        inp_lines = f.readlines()
    lines = []
    for x in inp_lines:
        if "#" in x:
            lines.append(x[:x.find("#")])
        else:
            lines.append(x)

    def command_from_str(x):
        cmd = x.split()
        if len(cmd) == 2 and cmd[0] == "push":
            push_val = int(cmd[1])
            return (stack_optimizer.optimize_number(int(cmd[1])) if optim else (["push "+str(push_val)] if push_val > 0 else ["push 1", "not", "push "+str(push_val), "sub"]))
        else:
            if len(x) > 0:
                return [x]
        return []

    blocks = dict()
    blocks_index = dict()
    b_id = 0
    for l in [x.split("\n") for x in "".join(lines).split("\n\n")]:
        cmds = []
        for x in l[1:]:
            cmds += command_from_str(x)
        blocks[l[0].split()[1]] = make_block(cmds)
        blocks_index[l[0].split()[1]] = b_id
        b_id += 1
    blocks["term"] = ([], [])
    blocks_index["term"] = b_id

    for x in blocks:
        if len(blocks[x][1]) == 1: # is goto
            goto_nr = blocks_index[blocks[x][1][0]]
            instrs = []
            if goto_nr > 100:
                instrs+=command_from_str("push 100")
                instrs+=command_from_str("push " + str(goto_nr // 100))
                instrs+=command_from_str("mul")
                instrs+=command_from_str("push " + str(goto_nr % 100))
                instrs+=command_from_str("add")
            else:
                instrs+=command_from_str("push " + str(goto_nr))
            goto_statement = make_block(instrs)[0]
            blocks[x] = (blocks[x][0] + ["⚪"] + goto_statement, []) # TODO: make goto use color, instead of white!

    # Split blocks
    j_width = 50
    right_line_gap = 2
    left_line_gap = 1
    for x in blocks:
        block_blocks = []
        last_offset = 0
        offset = 0
        running = True
        while running:
            last_offset = offset
            for j in range(j_width):
                if offset + j >= len(blocks[x][0]):
                    offset = len(blocks[x][0])
                    running = False
                    break
                if offset + j_width < len(blocks[x][0]):
                    if len(set(blocks[x][0][offset+j:offset+j_width])) == 2:
                        offset += j
                        break
            else:
                offset += j_width
            block_blocks.append(blocks[x][0][last_offset:offset+(1 if running else 0)])
        blocks[x] = (block_blocks, blocks[x][1])

    print ("Reading from:", i_file)
    print ("Saving to:", o_file)

    with open(o_file, 'w') as f:
        final_output = []

        total_block_width = (j_width+4+1)
        most_lines = max(len(blocks[x][0])for x in blocks) + 1
        for _ in range((most_lines + 1) // 2 * (right_line_gap+1) + (most_lines) // 2 * (left_line_gap+1) + 7 + 2):
            final_output.append(["⚪"]*(total_block_width*(len(blocks)-1)+20))


        for j, branch_instr in enumerate(make_block(["push 1","not"])[0]):
            final_output[0][j] = branch_instr
        final_output[1][0] = "⚫"
        final_output[2][5] = "⚫"
        final_output[4][4] = "⚫"
        final_output[3][3] = "⚫"

        def block_line_left(yi, xi, gap=2):
            final_output[yi][xi-3] = "⚫"
            final_output[yi-2][xi-2] = "⚫"
            final_output[yi-1][xi-1] = "⚫"
            final_output[yi+gap+2][xi-2] = "⚫"
            final_output[yi+gap+1][xi-4] = "⚫"
            final_output[yi+gap][xi-3] = "⚫"

        def block_line_right(yi, xi, gap=2):
            final_output[yi][xi+1] = "⚫"
            final_output[yi+gap+2][xi] = "⚫"

        for si, x in enumerate(blocks):
            start_index = 15 + total_block_width * si

            prepare_pointer = ["dup","push 1","not","greater","not","pointer","push 1","sub"]
            prepare_pointer_block = make_block(prepare_pointer)[0]
            prepare_pointer_index = prepare_pointer.index("pointer")+1
            for j, branch_instr in enumerate(prepare_pointer_block):
                final_output[0][start_index-prepare_pointer_index+j] = branch_instr

            li = 2 # line index

            if x == "term":
                final_output[li][start_index-1] = "🔴"
                final_output[li][start_index] = "🔴"
                final_output[li][start_index+1] = "🔴"
                final_output[li+1][start_index] = "🔴"

                final_output[li-1][start_index-1] = "⚫"
                final_output[li-1][start_index+1] = "⚫"
                final_output[li][start_index-2] = "⚫"
                final_output[li][start_index+2] = "⚫"
                final_output[li+1][start_index-1] = "⚫"
                final_output[li+1][start_index+1] = "⚫"
                final_output[li+2][start_index] = "⚫"

                # TODO: Make heart for term
                continue

            final_output[li+1][start_index] = "⚫"
            final_output[li][start_index-2] = "⚫"
            final_output[li-1][start_index-1] = "⚫"

            going_right = True
            for block_block in blocks[x][0]:
                if going_right:
                    for j, y in enumerate(block_block):
                        final_output[li][start_index+j] = y
                    block_line_right(li, start_index+j_width, gap=right_line_gap)
                    li = li + right_line_gap + 1
                else: # going left
                    for j, y in enumerate(block_block):
                        final_output[li][j_width+start_index-1-j] = y
                    block_line_left(li, start_index, gap=left_line_gap)
                    li = li + left_line_gap + 1
                going_right = not going_right

            if going_right:
                final_output[li][start_index+j_width+1] = "⚫"
            else:
                block_line_left(li, start_index, gap=left_line_gap)
                li = li + left_line_gap + 1
                final_output[li][start_index+j_width+1] = "⚫"
            #     block_line_left(li, start_index, gap=3)
            #     li = li + 3 + 1

            #     final_output[li-1][start_index+j_width] = "🔴"
            #     final_output[li][start_index+j_width] = "🔴"
            #     final_output[li+1][start_index+j_width] = "🔴"
            #     final_output[li][start_index+1+j_width] = "🔴"

            #     final_output[li-1][start_index-1+j_width] = "⚫"
            #     final_output[li-2][start_index+j_width] = "⚫"
            #     final_output[li-1][start_index+1+j_width] = "⚫"
            #     final_output[li][start_index+2+j_width] = "⚫"
            #     final_output[li+1][start_index+1+j_width] = "⚫"
            #     final_output[li+2][start_index+j_width] = "⚫"
            #     final_output[li+1][start_index-1+j_width] = "⚫"

        for x in final_output:
            f.write("".join(list(x)) + "\n")
