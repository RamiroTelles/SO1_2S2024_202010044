#!/bin/bash

images=("bajoc" "bajoc:2" "altoc:1" "altoc:2")
#echo ${images[ RANDOM % ${#images[@]}]}

for i in {1..10}
do

    docker run -d ${images[ RANDOM % ${#images[@]}]}
done    