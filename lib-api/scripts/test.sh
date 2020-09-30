#!/bin/sh

pid_path=/tmp/.oid.pid

call_clean() {
  local pid=$1

  echo [clean] Killing process with PID $pid...
  kill $pid

  echo [clean] Removing test sandbox store...
  rm -rf ~/.config/oi/.store/test

  echo [clean] Removing pid file itself...
  rm -f $pid_path

  echo [clean] Done.
}

call_test() {
  ../target/debug/oid --sandbox test --detach --pid $pid_path --port 9999 --workers 1
  local pid=$(cat $pid_path)
  cargo t -- --nocapture && call_clean $pid || call_clean $pid
}

call_test
