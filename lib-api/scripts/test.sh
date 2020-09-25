#!/bin/sh

call_clean() {
  local pid=$1

  echo [clean] Killing process with PID $pid...
  kill $pid

  echo [clean] Removing test sandbox store...
  rm -rf ~/.config/oi/.store/test

  echo [clean] Done.
}

call_test() {
  local pid_path=/tmp/.oid.pid
  ../target/debug/oid --sandbox test --detach --pid $pid_path
  local pid=$(cat $pid_path)
  cargo t -- --nocapture && call_clean $pid || call_clean $pid
}

call_test
