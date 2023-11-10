#!/bin/bash
version=2.3.0
cargo check && docker build --network=host . -t tnt:$version
kubectl set image  deployment/tnt tnt=tnt:$version
