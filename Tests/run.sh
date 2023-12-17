SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd );

results=()
while read -d ";" -r var; do
    filename=$(echo "$var" | cut -d "=" -f 1)
    expected=$(echo "$var" | cut -d "=" -f 2)
    echo "'$filename' expects '$expected'";
    python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" gen_all;
    result=$(python3 "$SCRIPT_DIR/../cli.py" "$SCRIPT_DIR/$filename.smpl" run_piet);
    echo "'$result'"
    if [ "$result" = "$expected" ]; then
	results+=(1)
    else
	results+=(0)
    fi
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
