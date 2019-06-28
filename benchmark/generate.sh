#!/bin/bash

if [[ "$#" != 2 ]]; then
  echo "$0 key-count op-count"
  exit 0
fi

KEY_COUNT="$1"
OP_COUNT="$2"

# Clear previous files.
rm -f keys ops ops-get ops-set

source "tqdm.sh"

# Generate random keys.
TQDM="$(get_tqdm_command "${KEY_COUNT}")"
echo "[$(date)] Generate random ${KEY_COUNT} keys"
for I in $(seq "${KEY_COUNT}"); do
  uuidgen -r >> keys
  echo "."
done | ${TQDM} > /dev/null

KEYS=($(cat keys))

# Generate random get/set operations.
TQDM="$(get_tqdm_command "${OP_COUNT}")"
echo "[$(date)] Generate random ${OP_COUNT} operations"
for I in $(seq "${OP_COUNT}"); do
  INDEX=$((${RANDOM} % ${KEY_COUNT}))
  KEY="${KEYS[INDEX]}"
  if [[ "$((${RANDOM} % 2))" == "0" ]]; then
    OP_CODE="GET"
    echo -e "${OP_CODE} ${KEY}\r" >> ops
  else
    OP_CODE="SET"
    echo -e "${OP_CODE} ${KEY} ${RANDOM}\r" >> ops
  fi
  echo "."
done | ${TQDM} > /dev/null

echo "[$(date)] Generate a list of get/set operations"
cat ops | grep "GET" > ops-get
cat ops | grep "SET" > ops-set

