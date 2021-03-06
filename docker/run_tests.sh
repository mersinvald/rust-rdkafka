#!/bin/bash

# This script is supposed to run inside the docker container.

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

INTEGRATION_TESTS="target/debug/test_"

echo -e "${GREEN}*** Clean previous build ***${NC}"
cp -r /mount/* /rdkafka
cd /rdkafka/rdkafka-sys/librdkafka
make clean > /dev/null 2>&1
cd /rdkafka/
rm target/debug/rdkafka-*
rm "$INTEGRATION_TESTS"*

echo -e "${GREEN}*** Inject system allocator ***${NC}"
sed -i "/\/\/>alloc_system/ c\#![feature(alloc_system)]\nextern crate alloc_system;" src/lib.rs

echo -e "${GREEN}*** Build tests ***${NC}"
cargo test --no-run
if [ "$?" != "0" ]; then
    echo -e "${RED}*** Failure during compilation ***${NC}"
    exit 1
fi

# UNIT TESTS

echo -e "${GREEN}*** Run unit tests ***${NC}"
valgrind --error-exitcode=100 --leak-check=full target/debug/rdkafka-* --nocapture

if [ "$?" != "0" ]; then
    echo -e "${RED}*** Failure in unit tests ***${NC}"
    exit 1
fi
echo -e "${GREEN}*** Unit tests succeeded ***${NC}"

# INTEGRATION TESTS

for test_file in `ls "$INTEGRATION_TESTS"*`
do
    echo -e "${GREEN}Executing "$test_file"${NC}"
    valgrind --error-exitcode=100 --leak-check=full "$test_file" --nocapture
    if [ "$?" != "0" ]; then
        echo -e "${RED}*** Failure in integration tests ***${NC}"
        exit 1
    fi
done

echo -e "${GREEN}*** Integration tests succeeded ***${NC}"
