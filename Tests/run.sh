SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd );

results=()
while read var; do
    set -- $var;
    echo "$1 is $2";
    python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$1.smpl" gen_all;
    result=$(python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$1.smpl" run_piet);
    echo $result;
    results+=($[ $result == $2 ])
done < $SCRIPT_DIR/test_results.txt;

counter=0
echo ""
echo -n "Results: "
for result in ${results[@]}; do
    if [ $result ];
    then echo -en "\e[0;32m"; echo -n "O";((counter=counter+1));
    else echo -en '\e[0;31m'; echo -n "X";
    fi; echo -en "\e[0;0m";
done
echo ""
echo "$counter/${#results[@]}"

# # python cli SCRIPT_DIR/$test
