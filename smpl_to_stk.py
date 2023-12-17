import sys

def dup_value_x_deep(instrs, index, x):
    # Get the value to the top
    instrs[index][1].append("push "+str(x))
    instrs[index][1].append("push -1")
    instrs[index][1].append("roll")

    instrs[index][1].append("dup")

    # put it back
    instrs[index][1].append("push "+str(x+1))
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

def swap(instrs, index):
    instrs[index][1].append("push 2")
    instrs[index][1].append("push -1")
    instrs[index][1].append("roll")

def dup_at_depth(instrs, index):
    # Save / update depth
    instrs[index][1].append("dup")
    instrs[index][1].append("push 1")
    instrs[index][1].append("add")

    # Fetch the element
    instrs[index][1].append("push -1")
    instrs[index][1].append("roll")

    # dup and save element
    instrs[index][1].append("dup")
    instrs[index][1].append("push 3")
    instrs[index][1].append("push -1")
    instrs[index][1].append("roll")

    # Put back the new element
    instrs[index][1].append("push 1")
    instrs[index][1].append("add")
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

# Put the second element on the stack, with element at depth given by the top element of the stack
def put_at_depth(instrs, index):
    # Put back the new element
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

# Swap the second element on the stack, with element at depth given by the top element of the stack
def swap_at_depth(instrs, index):
    # Save / update depth
    instrs[index][1].append("dup")
    instrs[index][1].append("push 1")
    instrs[index][1].append("add")

    # Fetch the element
    instrs[index][1].append("push -1")
    instrs[index][1].append("roll")

    # Do the swap
    instrs[index][1].append("push 3")
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

    # Put back the new element
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

def add(instrs, index, n):
    instrs[index][1].append("push "+str(n))
    instrs[index][1].append("add")

def sub(instrs, index, n):
    instrs[index][1].append("push "+str(n))
    instrs[index][1].append("sub")

def number_or_ord(v):
    return v if v.lstrip("-").isnumeric() else str(ord(v))

def eq(instrs, index):
    instrs[index][1].append("sub")
    instrs[index][1].append("dup")
    instrs[index][1].append("mul")
    instrs[index][1].append("push 0")
    instrs[index][1].append("greater")
    instrs[index][1].append("not")

def binop(instrs, index,op):
    instrs[index][1].append("push 3")
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")
    instrs[index][1].append(op)
    swap(instrs, index)
    instrs[index][1].append("push -1")
    instrs[index][1].append("add")

def goto_new_label(instrs, index):
    label_index = len(instrs)
    new_label = "l" + str(label_index)
    instrs[index][1].append("goto " + new_label)

    instrs.append((new_label, []))
    return label_index

def branch_new_labels(instrs, index):
    label1_index = len(instrs)
    new1_label = "l" + str(label1_index)
    instrs.append((new1_label, []))

    label2_index = len(instrs)
    new2_label = "l" + str(label2_index)
    instrs.append((new2_label, []))

    instrs[index][1].append("branch " + new1_label + " " + new2_label)

    return label1_index, label2_index

def get_offset_for_var_index(instrs, index, var_index):
    instrs[index][1].append("push "+str(var_index+1))
    return index

def handle_smpl_instr(var_list, instrs, index, l):
    next_index = index
    match l[0]:
        case "label":
            index = len(instrs)
            next_index = len(instrs)
            instrs.append((l[1], []))

        case "push":
            add(instrs, index,1)
            instrs[index][1].append("push " + number_or_ord(l[1]))
            swap(instrs, index)

        case "pop":
            swap(instrs, index)
            instrs[index][1].append("pop")
            sub(instrs, index,1)

        case "eq":
            swap(instrs, index)
            instrs[index][1].append("push " + number_or_ord(l[1]))
            eq(instrs, index)
            swap(instrs, index)

        case "not":
            swap(instrs, index)
            instrs[index][1].append("not")
            swap(instrs, index)

        case "add" | "greater" | "sub" | "div" | "mod" | "mul":
            binop(instrs, index, l[0])

        case "dup":
            swap(instrs, index)
            instrs[index][1].append("dup")
            instrs[index][1].append("push 3")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")
            instrs[index][1].append("push 1")
            instrs[index][1].append("add")

        case "inN":
            label_index = len(instrs)
            new_label = "l" + str(label_index)
            instrs[index][1].append("goto " + new_label)

            instrs.append((new_label, []))
            instrs[label_index][1].append("push -2")
            instrs[label_index][1].append("push -3")
            instrs[label_index][1].append("inN")
            swap(instrs, label_index)

            # Is it -3 ?
            instrs[label_index][1].append("push -3")
            eq(instrs, label_index)

            succ_label_index, fail_label_index = branch_new_labels(instrs, label_index)

            continue_label_index = len(instrs)
            continue_new_label = "l" + str(continue_label_index)
            instrs.append((continue_new_label, []))

            instrs[succ_label_index][1].append("push 3")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("roll")
            instrs[succ_label_index][1].append("pop")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("add")
            instrs[succ_label_index][1].append("goto " + continue_new_label)

            instrs[fail_label_index][1].append("pop")
            instrs[fail_label_index][1].append("goto " + continue_new_label)


            index = fail_label_index
            next_index = continue_label_index

        case "inC":
            label_index = len(instrs)
            new_label = "l" + str(label_index)
            instrs[index][1].append("goto " + new_label)

            instrs.append((new_label, []))
            instrs[label_index][1].append("push -2")
            instrs[label_index][1].append("push -3")
            instrs[label_index][1].append("inC")
            swap(instrs, label_index)

            # Is it -3 ?
            instrs[label_index][1].append("push -3")
            eq(instrs, label_index)

            succ_label_index, fail_label_index = branch_new_labels(instrs, label_index)

            continue_label_index = len(instrs)
            continue_new_label = "l" + str(continue_label_index)
            instrs.append((continue_new_label, []))

            instrs[succ_label_index][1].append("push 3")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("roll")
            instrs[succ_label_index][1].append("pop")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("add")
            instrs[succ_label_index][1].append("goto " + continue_new_label)

            instrs[fail_label_index][1].append("pop")
            instrs[fail_label_index][1].append("goto " + continue_new_label)

            index = fail_label_index
            next_index = continue_label_index

        case "goto":
            instrs[index][1].append("goto " + l[1])

        case "branch":
            instrs[index][1].append("push 1")
            instrs[index][1].append("sub")
            swap(instrs, index)
            instrs[index][1].append("branch " + l[1] + " " + l[2])

        case "set":
            assert (len(l) == 2)

            var_index = 0
            for i, x in enumerate(var_list):
                if x == l[1]:
                    var_index = i
                    break
            else:
                print ("Variable", l[1], "was not defined")
                exit(1)

            new_index = get_offset_for_var_index(instrs, index, var_index)
            swap(instrs, new_index)
            instrs[new_index][1].append("dup")

            instrs[new_index][1].append("push 3")
            instrs[new_index][1].append("push 1")
            instrs[new_index][1].append("roll")
            swap(instrs, new_index)
            instrs[new_index][1].append("sub")

            instrs[new_index][1].append("push 1")
            instrs[new_index][1].append("add")
            # Add 1?
            # instrs[new_index][1].append("push 1")
            # instrs[new_index][1].append("add")

            instrs[new_index][1].append("push 3")
            instrs[new_index][1].append("push -1")
            instrs[new_index][1].append("roll")

            swap(instrs, new_index)

            swap_at_depth(instrs, new_index)

            instrs[new_index][1].append("pop")
            instrs[new_index][1].append("push 1")
            instrs[new_index][1].append("sub")

            index = new_index
            next_index = new_index

        case "get":
            assert (len(l) == 2)

            var_index = 0
            for i, x in enumerate(var_list):
                if x == l[1]:
                    var_index = i
                    break
            else:
                print ("Variable", l[1], "was not defined")
                exit(1)

            new_index = get_offset_for_var_index(instrs, index, var_index)
            swap(instrs, new_index)
            instrs[new_index][1].append("dup")

            instrs[new_index][1].append("push 3")
            instrs[new_index][1].append("push 1")
            instrs[new_index][1].append("roll")
            swap(instrs, new_index)
            instrs[new_index][1].append("sub")

            # Add 1?
            instrs[new_index][1].append("push 1")
            instrs[new_index][1].append("add")

            dup_at_depth(instrs, new_index)

            swap(instrs, new_index)
            instrs[new_index][1].append("push 1")
            instrs[new_index][1].append("add")

            index = new_index
            next_index = new_index

        case "append":
            assert (len(l) == 1)

            swap(instrs, next_index)
            instrs[next_index][1].append("dup")

            # Is it -1?
            instrs[next_index][1].append("push -1")
            eq(instrs, next_index)

            suc_label_index, fail_label_index = branch_new_labels(instrs, next_index)

            ####################################
            # Success block = Initialize alloc #
            ####################################

            instrs[suc_label_index][1].append("pop")
            instrs[suc_label_index][1].append("push 3")
            swap(instrs, suc_label_index)
            _, next_index = handle_smpl_instr(var_list, instrs, suc_label_index, ["malloc"])

            instrs[next_index][1].append("dup")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            dup_at_depth(instrs, next_index)

            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("sub")

            swap(instrs, next_index)

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            swap(instrs, next_index)
            instrs[next_index][1].append("dup")

            instrs[next_index][1].append("push 4")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("roll")
            swap(instrs, next_index)

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            instrs[next_index][1].append("push 0")

            dup_value_x_deep(instrs, next_index, 3)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            instrs[next_index][1].append("push 1")

            instrs[next_index][1].append("push 5")
            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("roll")

            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("add")

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("push -1")
            instrs[next_index][1].append("roll")

            instrs[next_index][1].append("goto l" + str(fail_label_index))

            ###########################################
            # End of Success block = Initialize alloc #
            ###########################################

            instrs[fail_label_index][1].append("push 3")
            instrs[fail_label_index][1].append("push 1")
            instrs[fail_label_index][1].append("roll")

            dup_value_x_deep(instrs, fail_label_index, 3)
            swap(instrs, fail_label_index)
            instrs[fail_label_index][1].append("push 1")
            instrs[fail_label_index][1].append("add")

            dup_value_x_deep(instrs, fail_label_index, 2)
            swap(instrs, fail_label_index)

            instrs[fail_label_index][1].append("push 1")
            instrs[fail_label_index][1].append("add")

            _, next_index = handle_smpl_instr(var_list, instrs, fail_label_index, ["get_heap"])

            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("push -1")
            instrs[next_index][1].append("roll")

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

            dup_value_x_deep(instrs, next_index, 3)
            dup_value_x_deep(instrs, next_index, 3)
            instrs[next_index][1].append("greater")

            in_bounds_index, realloc_index = branch_new_labels(instrs, next_index)

            continue_label_index = len(instrs)
            continue_new_label = "l" + str(continue_label_index)
            instrs.append((continue_new_label, []))

            ###################
            # in bounds block #
            ###################

            instrs[in_bounds_index][1].append("push 3")
            instrs[in_bounds_index][1].append("push 1")
            instrs[in_bounds_index][1].append("roll")
            swap(instrs, in_bounds_index)

            instrs[in_bounds_index][1].append("pop")
            instrs[in_bounds_index][1].append("push 1")
            instrs[in_bounds_index][1].append("add")
            instrs[in_bounds_index][1].append("dup")

            instrs[in_bounds_index][1].append("push 3")
            instrs[in_bounds_index][1].append("push 2")
            instrs[in_bounds_index][1].append("roll")

            next_index = in_bounds_index
            dup_value_x_deep(instrs, next_index, 5)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

            dup_value_x_deep(instrs, next_index, 4)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("roll")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

            instrs[next_index][1].append("goto " + continue_new_label)

            ##########################
            # END of in bounds block #
            ##########################

            #################
            # realloc block #
            #################

            # instrs[realloc_index][1].append("push 3")
            swap(instrs, realloc_index)
            instrs[realloc_index][1].append("pop")
            swap(instrs, realloc_index)
            instrs[realloc_index][1].append("push 2")
            instrs[realloc_index][1].append("mul")
            instrs[realloc_index][1].append("dup")
            instrs[realloc_index][1].append("push 2")
            instrs[realloc_index][1].append("add")

            instrs[realloc_index][1].append("push 3")
            instrs[realloc_index][1].append("push 2")
            instrs[realloc_index][1].append("roll")

            _, next_index = handle_smpl_instr(var_list, instrs, realloc_index, ["malloc"])

            # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get", l[1]])

            dup_value_x_deep(instrs, next_index, 4)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            instrs[next_index][1].append("dup")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            dup_at_depth(instrs, next_index)

            instrs[next_index][1].append("push 2")
            instrs[next_index][1].append("sub")
            dup_value_x_deep(instrs, next_index, 4)
            instrs[next_index][1].append("sub")

            swap(instrs, next_index)

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set", l[1]])

            instrs[next_index][1].append("push 6")
            instrs[next_index][1].append("push -1")
            instrs[next_index][1].append("roll")
            instrs[next_index][1].append("pop")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")
            swap(instrs, next_index)

            instrs[next_index][1].append("push 5")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("roll")

            dup_value_x_deep(instrs, next_index, 2)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])
            swap(instrs, next_index)
            instrs[next_index][1].append("push 2")
            instrs[next_index][1].append("mul")
            swap(instrs, next_index)

            # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get", l[1]])

            dup_value_x_deep(instrs, next_index, 6)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

            swap(instrs, next_index)
            instrs[next_index][1].append("dup")

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("push 2")
            instrs[next_index][1].append("roll")

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

            instrs[next_index][1].append("push 0")
            swap(instrs, next_index)

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            dup_value_x_deep(instrs, next_index, 7)
            instrs[next_index][1].append("push 5")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("roll")

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            _, done_index = handle_smpl_instr(var_list, instrs, next_index, ["copy_memory"])

            instrs[done_index][1].append("push 5")
            instrs[done_index][1].append("push -1")
            instrs[done_index][1].append("roll")

            # Done index
            # _, next_index = handle_smpl_instr(var_list, instrs, done_index, ["get", l[1]])
            next_index = done_index
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])
            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("push -1")
            instrs[next_index][1].append("roll")
            instrs[next_index][1].append("pop")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")

            instrs[next_index][1].append("goto l" + str(in_bounds_index))

            ########################
            # end of realloc block #
            ########################

            index = next_index
            next_index = continue_label_index

        case "get_heap":
            assert (len(l) == 1)
            instrs[index][1].append("dup")
            instrs[index][1].append("push 1")
            instrs[index][1].append("add")

            dup_at_depth(instrs, index)
            dup_value_x_deep(instrs, index, 2)
            instrs[index][1].append("add")

            instrs[index][1].append("push 3")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")
            instrs[index][1].append("sub")

            dup_at_depth(instrs, index)
            swap(instrs, index)

            index = index
            next_index = index

        case "set_heap":
            assert (len(l) == 1)

            instrs[index][1].append("dup")
            instrs[index][1].append("push 1")
            instrs[index][1].append("add")

            dup_at_depth(instrs, index)
            dup_value_x_deep(instrs, index, 2)
            instrs[index][1].append("add")
            # instrs[index][1].append("push 1")
            # instrs[index][1].append("add")

            instrs[index][1].append("push 3")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")
            instrs[index][1].append("sub")

            instrs[index][1].append("push 3")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")

            swap(instrs, index)
            swap_at_depth(instrs, index)

            instrs[index][1].append("pop")
            instrs[index][1].append("push 2")
            instrs[index][1].append("sub")

            index = index
            next_index = index

        case "get_elem":
            assert (len(l) == 1)
            _, next_index = handle_smpl_instr(var_list, instrs, index, ["push","2"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["add"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["add"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

            index = next_index
            next_index = next_index

        case "set_elem":
            _, next_index = handle_smpl_instr(var_list, instrs, index, ["get", l[1]])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push","2"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["add"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["add"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

            index = next_index
            next_index = next_index

        case "get_list":
            _, next_index = handle_smpl_instr(var_list, instrs, index, ["dup"])
            _, next_index = handle_smpl_instr(var_list, instrs, index, ["length"])

            label_index = goto_new_label(instrs, next_index) # move all elements to new array
            _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["push", "1"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["sub"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq", "-1"])

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")

            swap(instrs, next_index)

            return_index, in_bounds_index = branch_new_labels(instrs, next_index)

            _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["dup"])

            dup_value_x_deep(instrs, next_index, 4)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_elem"])

            swap(instrs, next_index)
            instrs[next_index][1].append("push 4")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("roll")

            instrs[next_index][1].append("goto l" + str(label_index))

            _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])

            index = next_index
            next_index = next_index

        case "print_listC":
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_list"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["length"])

            label_index = goto_new_label(instrs, next_index) # move all elements to new array
            _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["push", "1"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["sub"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq", "-1"])

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")

            swap(instrs, next_index)
            return_index, in_bounds_index = branch_new_labels(instrs, next_index)

            instrs[in_bounds_index][1].append("push 3")
            instrs[in_bounds_index][1].append("push -1")
            instrs[in_bounds_index][1].append("roll")
            swap(instrs, in_bounds_index)
            _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["outC"])

            instrs[next_index][1].append("goto l" + str(label_index))

            _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])

            index = next_index
            next_index = next_index

        case "print_listN":
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push",str(ord("["))])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_list"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["length"])

            label_index = goto_new_label(instrs, next_index) # move all elements to new array
            _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["push", "1"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["sub"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq", "-1"])

            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")

            swap(instrs, next_index)
            return_index, in_bounds_index = branch_new_labels(instrs, next_index)

            instrs[in_bounds_index][1].append("push 3")
            instrs[in_bounds_index][1].append("push -1")
            instrs[in_bounds_index][1].append("roll")
            swap(instrs, in_bounds_index)
            _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["outN"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push",str(ord(","))])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

            instrs[next_index][1].append("goto l" + str(label_index))

            _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push",str(ord("]"))])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

            index = next_index
            next_index = next_index

        case "readC_until":
            label_index = goto_new_label(instrs, index)
            _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["inC"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq", l[1]])
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")
            swap(instrs, next_index)
            done_index, append_index = branch_new_labels(instrs, next_index)

            dup_value_x_deep(instrs, append_index, 3)
            swap(instrs, append_index)
            instrs[append_index][1].append("push 1")
            instrs[append_index][1].append("add")
            _, next_index = handle_smpl_instr(var_list, instrs, append_index, ["append"])
            instrs[next_index][1].append("push 3")
            instrs[next_index][1].append("push -1")
            instrs[next_index][1].append("roll")
            instrs[next_index][1].append("pop")
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("sub")

            instrs[next_index][1].append("goto l" + str(label_index))


            _, next_index = handle_smpl_instr(var_list, instrs, done_index, ["pop"])
            instrs[next_index][1].append("debug")

            index = next_index
            next_index = next_index

        case "outC":
            swap(instrs, index)
            instrs[index][1].append("outC")
            instrs[index][1].append("push 1")
            instrs[index][1].append("sub")

        case "outN":
            swap(instrs, index)
            instrs[index][1].append("outN")
            instrs[index][1].append("push 1")
            instrs[index][1].append("sub")


        case "roll":
            dup_value_x_deep(instrs, index, 2)
            dup_value_x_deep(instrs, index, 4)
            instrs[index][1].append("mod")

            instrs[index][1].append("push 3")
            instrs[index][1].append("add")
            instrs[index][1].append("push 1")
            instrs[index][1].append("roll")

            dup_value_x_deep(instrs, index, 2)
            instrs[index][1].append("mod")

            swap(instrs, index)
            instrs[index][1].append("push 1")
            instrs[index][1].append("add")
            swap(instrs, index)

            instrs[index][1].append("roll")
            instrs[index][1].append("push 2")
            instrs[index][1].append("sub")

        case "malloc":
            instrs[index][1].append("dup")

            instrs[index][1].append("push 3")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")

            instrs[index][1].append("dup")
            instrs[index][1].append("dup")

            instrs[index][1].append("push 4")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")
            instrs[index][1].append("add")
            instrs[index][1].append("push 1")
            instrs[index][1].append("add")

            swap(instrs, index)
            instrs[index][1].append("dup")

            label_index = goto_new_label(instrs, index)
            instrs[label_index][1].append("dup")
            instrs[label_index][1].append("push 0")
            instrs[label_index][1].append("greater")
            loop_index, roll_index = branch_new_labels(instrs, label_index)

            instrs[loop_index][1].append("push 0")

            instrs[loop_index][1].append("push 4")
            instrs[loop_index][1].append("push 1")
            instrs[loop_index][1].append("roll")

            instrs[loop_index][1].append("push 1")
            instrs[loop_index][1].append("sub")
            instrs[loop_index][1].append("goto " + "l" + str(label_index))

            instrs[roll_index][1].append("pop")
            instrs[roll_index][1].append("roll")

            dup_value_x_deep(instrs, roll_index, 2)

            # # Add 1 extra??
            instrs[roll_index][1].append("push 1")
            instrs[roll_index][1].append("add")

            dup_at_depth(instrs, roll_index)
            instrs[roll_index][1].append("add")

            dup_value_x_deep(instrs, roll_index, 2)
            # # Add 1 extra??
            instrs[roll_index][1].append("push 1")
            instrs[roll_index][1].append("add")
            swap_at_depth(instrs, roll_index)
            instrs[roll_index][1].append("pop")

            # Decrement stack size
            instrs[roll_index][1].append("push 1")
            instrs[roll_index][1].append("sub")

            index = roll_index
            next_index = roll_index

        case "length":
            assert (len(l) == 1)
            swap(instrs, index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)
            index, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

        case "copy_memory":
            label_index = goto_new_label(instrs, index) # move all elements to new array

            dup_value_x_deep(instrs, label_index, 3)
            dup_value_x_deep(instrs, label_index, 3)
            eq(instrs, label_index)

            done_index, loop_index = branch_new_labels(instrs, label_index)

            dup_value_x_deep(instrs, loop_index, 4)
            dup_value_x_deep(instrs, loop_index, 3)
            instrs[loop_index][1].append("add")
            instrs[loop_index][1].append("push 2")
            instrs[loop_index][1].append("add")

            swap(instrs, loop_index)
            instrs[loop_index][1].append("push 1")
            instrs[loop_index][1].append("add")

            _, next_index = handle_smpl_instr(var_list, instrs, loop_index, ["get_heap"])

            # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get", l[1]])
            dup_value_x_deep(instrs, next_index, 6)
            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")

            swap(instrs, next_index)
            dup_value_x_deep(instrs, next_index, 4)
            instrs[next_index][1].append("add")
            instrs[next_index][1].append("push 2")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)
            _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

            swap(instrs, next_index)
            instrs[next_index][1].append("push 1")
            instrs[next_index][1].append("add")
            swap(instrs, next_index)

            instrs[next_index][1].append("goto l" + str(label_index))

            index = next_index
            next_index = done_index

        case "debug":
            instrs[index][1].append("debug")

        case default:
            print ("Did not find", l)

    return index, next_index

def smpl_to_stk(i_file, o_file):
    inp_lines = []
    with open(i_file, 'r') as f:
        inp_lines = f.readlines()
    inp_lines = list(map(lambda x: x.split(), inp_lines))

    instrs = [("main",["push 0", "push 1"])]

    data_type_size = {"num": 1, "list": 0}

    index = 0
    label_count = 0
    var_list = []

    for inp_line_index, l in enumerate(inp_lines):
        if len(l) < 1 or l[0] != "var":
            break

        var_list = [l[1]] + var_list

        # Allocate empty variable
        instrs[index][1].append("push " + ("0" if l[2] == "num" else "-1"))

        ####################
        # Fetch stack size #
        ####################
        swap(instrs, index)
        # Update size
        instrs[index][1].append("push 1")
        instrs[index][1].append("add")

        instrs[index][1].append("dup")

        instrs[index][1].append("push 3")
        instrs[index][1].append("push 1")
        instrs[index][1].append("roll")

        ######################
        # Rotate into bottom #
        ######################
        instrs[index][1].append("push 1")
        instrs[index][1].append("roll")

    for l in inp_lines[inp_line_index:]:
        if len(l) == 0 or l[0] == "#":
            continue
        instrs[index][1].append("#+" + " ".join(l))
        last_index, index = handle_smpl_instr(var_list, instrs, index, l)
        instrs[last_index][1].append("#-" + " ".join(l))

    with open(o_file, 'w') as f:
        f.write("\n\n".join(["\n".join(["label " + label] + instr) for label, instr in instrs]))
