#!/bin/bash
version=2.1.9
cargo check && docker build --network=host . -t tnt:$version
kubectl set image  deployment/tnt tnt=tnt:$version
