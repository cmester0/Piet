import sys
import png

from generate_number import * # optimize number

def stk_to_ascii_piet(i_file, o_file, optim=True):
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
            case "nop":
                output.append("⚪")
                output.append("🔴")
                return
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
                return
        output.append(colors[previous_r][previous_c])

    def make_block(block):
        output = ["🔴"]
        for inp in block:
            cmd = inp.split()
            match cmd[0]:
                case "branch":
                    # # pointer
                    # previous_c, previous_r = rev_map[output[-1]]
                    # previous_c = (previous_c + 3) % 6
                    # previous_r = (previous_r + 1) % 3
                    # output.append(colors[previous_r][previous_c])
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
    inv_block_index = []
    b_id = 0

    block_cmds = [x.split("\n") for x in "".join(lines).split("\n\n")]

    b_width = math.ceil(math.sqrt(len(block_cmds)))
    b_height = math.ceil(len(block_cmds) // b_width) + 1

    def id_to_coord(b_id):
        return (b_id % b_width, b_id // b_width)

    for l in block_cmds:
        cmds = []
        for x in l[1:]:
            cmds += command_from_str(x)
        blocks[l[0].split()[1]] = make_block(cmds)
        blocks_index[l[0].split()[1]] = b_id
        inv_block_index.append(l[0].split()[1])
        b_id += 1
    blocks["term"] = (make_block([]), [])
    blocks_index["term"] = b_id
    inv_block_index.append("term")

    for x in blocks:
        if len(blocks[x][1]) == 1: # is goto
            b_id = blocks_index[blocks[x][1][0]]
            b_x, b_y = id_to_coord(b_id)
            goto_statement = make_block(command_from_str("push " + str(b_x))+command_from_str("push " + str(b_y)))[0]
            blocks[x] = (blocks[x][0], [goto_statement]) # TODO: make goto use color, instead of white!
        if len(blocks[x][1]) == 2:
            b_id_1 = blocks_index[blocks[x][1][0]]
            b_id_2 = blocks_index[blocks[x][1][1]]

            b_1_x, b_1_y = id_to_coord(b_id_1)
            b_2_x, b_2_y = id_to_coord(b_id_2)

            goto_statement_1 = make_block(["pointer"]+["nop"]+command_from_str("push " + str(b_1_x))+command_from_str("push " + str(b_1_y)))[0]
            goto_statement_2 = make_block(["pointer"]+["nop"]+command_from_str("push " + str(b_2_x))+command_from_str("push " + str(b_2_y)))[0]
            blocks[x] = (blocks[x][0], [goto_statement_1, goto_statement_2]) # TODO: make goto use color, instead of white!

    # Split blocks
    j_width = max(30, max(b_width,b_height)//5)
    right_line_gap = 2
    left_line_gap = 1

    def split_in_blocks(to_split, going_right=True):
        block_blocks = []
        last_offset = 0
        offset = 0
        running = True
        going_right = not going_right
        while running:
            last_offset = offset
            for j in range(j_width):
                if offset + j >= len(to_split):
                    offset = len(to_split)
                    running = False
                    break
                if offset + j_width < len(to_split):
                    if len(set(to_split[offset+j:offset+j_width])) == 2:
                        offset += j
                        break
            else:
                offset += j_width
            block_blocks.append(to_split[last_offset:offset+(1 if running else 0)])
            going_right = not going_right
        return block_blocks, going_right

    for x in blocks:
        splits, going_right = split_in_blocks(blocks[x][0])
        branch_blocks = []
        if len(blocks[x][1]) == 1:
            goto_splits, _ = split_in_blocks(blocks[x][1][0])
            splits += goto_splits
        elif len(blocks[x][1]) == 2:
            if going_right:
                splits.append([])
            goto_splits, _ = split_in_blocks(blocks[x][1][1])
            splits += goto_splits
            branch_blocks = blocks[x][1][0]

        blocks[x] = (splits, branch_blocks)

    print ("Reading from:", i_file)
    print ("Saving to:", o_file)

    with open(o_file, 'w') as f:
        final_output = []

        total_block_width = (j_width+4+1) + 6
        total_block_height = max(
            (len(blocks[x][0]) + 1) // 2 * (right_line_gap + int(len(blocks[x][1]) > 0)) + \
            (len(blocks[x][0])) // 2 * (left_line_gap + int(len(blocks[x][1]) > 0)) + \
            (len(blocks[x][1][2:]) if len(blocks[x][1]) > 0 else 0)
            for x in blocks) + 4 + 30 # TODO: Fix

        pre = "⚪" * 8
        post = "⚪" * 1
        final_output.append(list("⚪" * (len(pre) + total_block_width * b_width + len(post))))
        final_output.append(list("⚫" * (len(pre) + total_block_width * b_width) + post))
        final_output.append(list("⚪" * (len(pre) + total_block_width * b_width + len(post))))
        for _ in range(b_height):
            final_output.append(list("⚪" * (len(pre)-1) + "⚫" + "⚫" * (total_block_width * b_width) + post))
            for _ in range(total_block_height-3):
                sym_line = pre
                for _ in range(b_width):
                    for _ in range(total_block_width-1):
                        sym_line += "⚪"
                    sym_line += "⚪" # ⚫
                sym_line += post
                final_output.append(list(sym_line))
            final_output.append(list(pre + "⚫" * (total_block_width * b_width) + post))
            final_output.append(list("⚪" * (len(pre)-2) + "⚫⚪" + "⚪" * (total_block_width * b_width) + post))

        for _ in range(15):
            final_output.append(list("⚪" * (len(pre) + total_block_width * b_width + len(post))))

        for j, branch_instr in enumerate(make_block(["push 1","not","push 1","not"])[0]):
            final_output[0][j] = branch_instr

        prepare_pointer = ["dup","push 1","not","greater","not","pointer","push 1","sub"]
        prepare_pointer_index = prepare_pointer.index("pointer")+1

        prepare_pointer_block = make_block(prepare_pointer)[0]
        prepare_pointer_pop = make_block(prepare_pointer[:prepare_pointer_index]+["pop"])[0][prepare_pointer_index+1:]

        for li in range(b_height):
            for j, branch_instr in enumerate(prepare_pointer_block):
                final_output[1+total_block_height*li+total_block_height-prepare_pointer_index+j+1][len(pre) + total_block_width * b_width] = branch_instr
            for j, y in enumerate(prepare_pointer_pop):
                final_output[1+total_block_height*li+total_block_height+1][len(pre) + total_block_width * b_width - 1 - j] = y

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
            start_index = 15 + total_block_width * (si % b_width)
            li = 7 + total_block_height * (si // b_width) # line index

            for j, y in enumerate(prepare_pointer_block):
                final_output[li-3][start_index-prepare_pointer_index-1+j] = y
            for j, y in enumerate(prepare_pointer_pop):
                final_output[li-3+1+j][start_index-1] = y

            if x == "term":
                for j, y in enumerate(blocks[x][0][0][0]):
                    final_output[li+j][start_index-1] = y

                heart_x, heart_y = li+1+len(blocks[x][0][0][0]), start_index-1

                final_output[heart_x][heart_y-1] = "🔴"
                final_output[heart_x][heart_y] = "🔴"
                final_output[heart_x][heart_y+1] = "🔴"
                final_output[heart_x+1][heart_y] = "🔴"

                final_output[heart_x-1][heart_y-1] = "⚫"
                final_output[heart_x-1][heart_y+1] = "⚫"
                final_output[heart_x][heart_y-2] = "⚫"
                final_output[heart_x][heart_y+2] = "⚫"
                final_output[heart_x+1][heart_y-1] = "⚫"
                final_output[heart_x+1][heart_y+1] = "⚫"
                final_output[heart_x+2][heart_y] = "⚫"
                # TODO: Make heart for term
                continue

            final_output[li+1][start_index-1] = "⚫"
            final_output[li][start_index-3] = "⚫"
            final_output[li-1][start_index-2] = "⚫"

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

            if len(blocks[x][1]) > 0:
                for j, y in enumerate(blocks[x][1][2:]):
                    final_output[li-right_line_gap+j][start_index+1] = y

            if not going_right:
                final_output[li+1][start_index+j_width] = "⚪" # 🔴
            else:
                final_output[li+1][start_index-2] = "⚪" # 🔴

        for x in final_output:
            f.write("".join(list(x)) + "\n")
