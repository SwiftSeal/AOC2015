in="1113222113"; 
for f in $(seq 1 100) ; do 
    in=$(echo "$in" | fold -w1 | uniq -c | tr '\n' ' ' | tr -d ' '); 
    echo $in | tr -d '\n' | wc -c; 
done
