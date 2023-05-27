#!/bin/bash
version=2.0.8
cargo check && docker build . -t tnt:$version
kubectl set image  deployment/tnt tnt=tnt:$version
