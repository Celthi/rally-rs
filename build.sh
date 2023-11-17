#!/bin/bash
version=2.3.13
docker build --network=host . -t tnt:$version
sleep 1 # wait for docker to finish publish image
if [[ $? -ne 0 ]]; then
    echo "Build failed"
    exit 1
fi
if [[ -z $VIEW ]]; then
    kubectl set image  deployment/tnt tnt=tnt:$version
else
    $VIEW/kubectl set image  deployment/tnt tnt=tnt:$version
fi
