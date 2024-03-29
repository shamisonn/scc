#!/bin/bash
try() {
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.s
  gcc-9 -o tmp tmp.s
  chmod +x tmp
  ./tmp y
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

try 21 "5+20-4"
try 21 "5 + 20 - 4"
try 21 "
5 + 20 - 4"

echo OK