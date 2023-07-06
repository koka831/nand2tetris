#!/usr/bin/env bash

# $ ./test.sh
# verify hack vm translator

set -u

GRAY='\033[1;90m'
RED='\033[1;31m'
GREEN='\033[1;32m'
RESET='\033[0m'

PROJECT="$PWD/hack-vm"
EMU="$PWD/tools/CPUEmulator.sh"

function info() { echo -e "${GRAY}info${RESET}: $1"; }
function success() { echo -e "${GREEN}success${RESET}: $1"; }
function error() { echo -e "${RED}failed${RESET}: $1"; }
function border() { echo -e "${GRAY}--------------------------------------------------------${RESET}"; }

# store bitwise OR of each tests exit code (so that it'll return `1` if one or more tests get failed)
status=0

# run_test ${compile_target} ${test_harness}
# @param compile_target vm file or directory contains vm files
# @param test_harness *.tst file
#
# download test suite from https://www.nand2tetris.org/software
function run_test() {
    local compile_target="$1"
    local test_harness="$2"
    local result

    local compile_target_path="${PROJECT}/tests/fixtures/${compile_target}"
    local test_harness_path="${PROJECT}/tests/fixtures/${test_harness}"

    info "compiling ${compile_target}..."
    result=$(cargo run --quiet -p hack-vm -- "${compile_target_path}" > "${test_harness_path%.*}.asm")
    local st=$?
    if [[ $st -ne 0 ]]; then
        error "Failed to compile ${compile_target}."
        error "$result"
        status=$(( status | st ))
    fi

    info "verifying ${compile_target}..."
    result=$(${EMU} "${test_harness_path}" 2>&1)
    st=$?
    if [[ $st -ne 0 ]]; then
        error "${compile_target}"
        error "$result"
        info "expected:"
        cat "${test_harness_path%.*}.cmp"
        info "actual:"
        cat "${test_harness_path%.*}.out"
        status=$(( status | st ))
    else
        success "${compile_target}"
    fi

    border
}

info "Running tests..."
# Stage.1
run_test "StackArithmetic/SimpleAdd/SimpleAdd.vm" "StackArithmetic/SimpleAdd/SimpleAdd.tst"
run_test "StackArithmetic/SimpleAdd" "StackArithmetic/SimpleAdd/SimpleAdd.tst"
run_test "StackArithmetic/StackTest/StackTest.vm" "StackArithmetic/StackTest/StackTest.tst"
run_test "StackArithmetic/StackTest" "StackArithmetic/StackTest/StackTest.tst"
# Stage.2
run_test "MemoryAccess/BasicTest/BasicTest.vm" "MemoryAccess/BasicTest/BasicTest.tst"
run_test "MemoryAccess/PointerTest/PointerTest.vm" "MemoryAccess/PointerTest/PointerTest.tst"
run_test "MemoryAccess/StaticTest/StaticTest.vm" "MemoryAccess/StaticTest/StaticTest.tst"
# Stage.3
run_test "ProgramFlow/BasicLoop/BasicLoop.vm" "ProgramFlow/BasicLoop/BasicLoop.tst"
run_test "ProgramFlow/FibonacciSeries/FibonacciSeries.vm" "ProgramFlow/FibonacciSeries/FibonacciSeries.tst"
run_test "FunctionCalls/SimpleFunction/SimpleFunction.vm" "FunctionCalls/SimpleFunction/SimpleFunction.tst"
run_test "FunctionCalls/NestedCall/Sys.vm" "FunctionCalls/NestedCall/NestedCall.tst"
run_test "FunctionCalls/NestedCall" "FunctionCalls/NestedCall/NestedCall.tst"
run_test "FunctionCalls/StaticsTest" "FunctionCalls/StaticsTest/StaticsTest.tst"
run_test "FunctionCalls/FibonacciElement" "FunctionCalls/FibonacciElement/FibonacciElement.tst"

exit "$status"
