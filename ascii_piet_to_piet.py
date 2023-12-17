import png

def ascii_piet_to_piet(i_file, o_file):
    # ⚪

    # ❤🧡💛💚💙💜
    # 🔴🟠🟡🟢🔵🟣
    # 🟥🟧🟨🟩🟦🟪

    # ⚫

    # FFFFFF

    # FFC0C0 FFFFC0 C0FFC0 C0FFFF C0C0FF FFC0FF
    # FF0000 FFFF00 00FF00 00FFFF 0000FF FF00FF
    # C00000 C0C000 00C000 00C0C0 0000C0 C000C0

    # 000000

    lines = []
    with open(i_file, 'r') as f:
        lines = [x[:-1] for x in f.readlines()]

    img = []
    for x in lines:
        l = []
        for c in x:
            match c:
               # FFFFFF
                case "⚪":
                    l.append([255, 255, 255])

                # FFC0C0 FFFFC0 C0FFC0 C0FFFF C0C0FF FFC0FF
                case "❤":
                    l.append([255, 192, 192])
                case "🧡":
                    l.append([255, 255, 192])
                case "💛":
                    l.append([192, 255, 192])
                case "💚":
                    l.append([192, 255, 255])
                case "💙":
                    l.append([192, 192, 255])
                case "💜":
                    l.append([255, 192, 255])

                # FF0000 FFFF00 00FF00 00FFFF 0000FF FF00FF
                case "🔴":
                    l.append([255,   0,   0])
                case "🟠":
                    l.append([255, 255,   0])
                case "🟡":
                    l.append([  0, 255,   0])
                case "🟢":
                    l.append([  0, 255, 255])
                case "🔵":
                    l.append([  0,   0, 255])
                case "🟣":
                    l.append([255,   0, 255])

                # C00000 C0C000 00C000 00C0C0 0000C0 C000C0
                case "🟥":
                    l.append([192,   0,   0])
                case "🟧":
                    l.append([192, 192,   0])
                case "🟨":
                    l.append([  0, 192,   0])
                case "🟩":
                    l.append([  0, 192, 192])
                case "🟦":
                    l.append([  0,   0, 192])
                case "🟪":
                    l.append([192,   0, 192])

                # 000000
                case "⚫":
                    l.append([  0,   0,   0])
        t = []
        for x in l:
            for y in x:
                t.append(y)
        img.append(t)

    # if debug:
    #     print ("Reading from:", i_file)
    #     print ("Saving to:", o_file)

    with open(o_file, 'wb') as f:
        w = png.Writer(len(img[0]) // 3, len(img), greyscale=False)
        w.write(f, img)

# import sys
# if __name__ == "__main__":
#     ascii_piet_to_piet(sys.argv[1], sys.argv[2])
