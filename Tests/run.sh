SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd );

inp=()
while read -d ";" -r var; do
    inp+=("$var")
done < $SCRIPT_DIR/test_results.txt;

echo "==============="
echo "Generate stk translation"
echo "==============="
for ((i = 0; i < ${#inp[@]}; i++)) do
    var=${inp[$i]};
    filename=$(echo "$var" | cut -d "=" -f 1)
    echo "Generating '$filename'";
    python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" gen_stk;
done
echo ""

echo "==============="
echo "Run tests with stk-interpreter"
echo "==============="
results_stk=()
for ((i = 0; i < ${#inp[@]}; i++)) do
    var=${inp[$i]};
    filename=$(echo "$var" | cut -d "=" -f 1)
    expected=$(echo "$var" | cut -d "=" -f 2)
    stdin=$(echo "$var" | cut -d "=" -f 3)

    echo "'$filename' expects '$expected' on input '$stdin'";
    result_stk=$(echo "$stdin" | python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" run_stk);
    echo "'$result_stk'"
    if [ "$result_stk" = "$expected" ]; then
        results_stk+=(1)
    else
        results_stk+=(0)
    fi
done

counter=0
echo ""
echo "---------------"
echo "Results (stk):"
echo "---------------"
for result in ${results_stk[@]}; do
    if [ $result ];
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
for ((i = 0; i < ${#inp[@]}; i++)) do
    var=${inp[$i]};
    filename=$(echo "$var" | cut -d "=" -f 1)
    echo "Generating '$filename'";
    python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" gen_piet;
done
echo ""

echo "==============="
echo "Run tests with piet-interpreter"
echo "==============="
results_piet=()
for ((i = 0; i < ${#inp[@]}; i++)) do
    var=${inp[$i]};
    filename=$(echo "$var" | cut -d "=" -f 1)
    expected=$(echo "$var" | cut -d "=" -f 2)
    stdin=$(echo "$var" | cut -d "=" -f 3)

    echo "'$filename' expects '$expected' on input '$stdin'";
    result_piet=$(echo "$stdin" | python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" run_piet);
    echo "'$result_piet'"
    if [ "$result_piet" = "$expected" ]; then
        results_piet+=(1)
    else
        results_piet+=(0)
    fi
done

counter=0
echo ""
echo "---------------"
echo "Results (piet):"
echo "---------------"
for result in ${results_piet[@]}; do
    if [ $result ];
    then echo -en "\e[0;32m"; echo -n "O";((counter=counter+1));
    else echo -en '\e[0;31m'; echo -n "X";
    fi; echo -en "\e[0;0m";
done
echo ""
echo "$counter/${#results_piet[@]}"
