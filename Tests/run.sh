SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd );

filenames=()
expectations=()
inputs=()
while read -d ";" -r var; do
    filename="$(echo -n "$var" | tr "\n" "\0" | cut -d "=" -f 1 | tr "\0" "\n")";
    expected="$(echo -n "$var" | tr "\n" "\0" | cut -d "=" -f 2 | tr "\0" "\n")";
    input="$(echo -n "$var" | tr "\n" "\0" | cut -d "=" -f 3 | tr "\0" "\n")";

    filenames+=("$filename");
    expectations+=("$expected");
    inputs+=("$input");
done < $SCRIPT_DIR/test_results.txt;

echo "==============="
echo "Generate stk translation"
echo "==============="
for ((i = 0; i < ${#filenames[@]}; i++)) do
    filename="${filenames[$i]}";
    echo "Generating '$filename'";
    python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" gen_stk;
done
echo ""

echo "==============="
echo "Run tests with stk-interpreter"
echo "==============="
results_stk=()
for ((i = 0; i < ${#filenames[@]}; i++)) do
    filename="${filenames[$i]}"
    expected="${expectations[$i]}"
    stdin="${inputs[$i]}"

    echo "Filename: '$filename'"; # expects '$expected' on input '$stdin'
    result_stk=$(echo "$stdin" | python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" run_stk);
    echo "Result: '$result_stk'"
    if [ "$result_stk" == "$expected" ]; then
        results_stk+=(1)
    else
	echo "Expected value: '$expected'";
        results_stk+=(0)
    fi
    echo ""
done

counter=0
echo "---------------"
echo "Results (stk):"
echo "---------------"
for result in ${results_stk[@]}; do
    if [ "$result" == "1" ];
    then echo -en "\e[0;32m"; echo -n "O";((counter=counter+1));
    else echo -en '\e[0;31m'; echo -n "X";
    fi; echo -en "\e[0;0m";
done
echo ""
echo "$counter/${#results_stk[@]}"

################################################
echo ""
################################################

echo "==============="
echo "Generate piet translation"
echo "==============="
for ((i = 0; i < ${#filenames[@]}; i++)) do
    filename="${filenames[$i]}";
    echo "Generating '$filename'";
    python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" gen_piet;
done
echo ""

echo "==============="
echo "Run tests with piet-interpreter"
echo "==============="
results_piet=()
for ((i = 0; i < ${#filenames[@]}; i++)) do
    filename="${filenames[$i]}"
    expected="${expectations[$i]}"
    stdin="${inputs[$i]}"

    echo "Filename: '$filename'"; # expects '$expected' on input '$stdin'
    result_piet=$(echo "$stdin" | python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" run_piet);
    echo "Result: '$result_piet'"
    if [ "$result_piet" == "$expected" ]; then
        results_piet+=(1)
    else
	echo "Expected value: '$expected'";
        results_piet+=(0)
    fi
    echo ""
done

counter=0
echo "---------------"
echo "Results (piet):"
echo "---------------"
for result in ${results_piet[@]}; do
    if [ "$result" == "1" ];
    then echo -en "\e[0;32m"; echo -n "O";((counter=counter+1));
    else echo -en '\e[0;31m'; echo -n "X";
    fi; echo -en "\e[0;0m";
done
echo ""
echo "$counter/${#results_piet[@]}"
