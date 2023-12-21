echo 'fn next_state(&self, input: Token) -> (Self, Option<Output>) {' >> parser_state.rs
echo '    match self {' >> parser_state.rs

previous_state=""

while IFS="," read -r state input next_state output
do
    # Remove double quotes
    state="${state//\"}"
    input="${input//\"}"
    next_state="${next_state//\"}"
    output="${output//\"}"

    if [[ "$state" != "$previous_state" ]]; then
        if [[ "$previous_state" != "" ]]; then
            echo '            _ => (Err, None),' >> parser_state.rs
            echo '        },' >> parser_state.rs
        fi
        echo "        $state => match input {" >> parser_state.rs
        previous_state="$state"
    fi

    if [[ "$output" == "" ]]; then
        echo "            $input => ($next_state, None)," >> parser_state.rs
    else
        echo "            $input => ($next_state, Some($output))," >> parser_state.rs
    fi
done < temp.csv

echo '            _ =>
