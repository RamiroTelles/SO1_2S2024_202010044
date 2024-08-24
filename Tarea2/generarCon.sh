#!/bin/bash



for i in {1..10}
do

    docker run -d --name py_con$RANDOM bajoc
done    