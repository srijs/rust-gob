#!/usr/bin/env bash

set -e

for GEN in `ls input`;
do go run "input/$GEN" > "output/${GEN}b";
done
