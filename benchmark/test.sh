#!/bin/bash

TARGET_HOST="${TARGET_HOST:-"localhost"}"
TARGET_PORT="${TARGET_PORT:-"6379"}"

if [[ "$#" < 1 ]]; then
  echo "$0 op-file iteration"
  exit 0
fi

INPUT="$1"
ITERATION="${2:-"10"}"

if [ ! -f "${INPUT}" ]; then
  echo "Invalid file: ${INPUT}"
  exit 1
fi

COUNT="$(cat "${INPUT}" | wc -l)"
TOTAL_COUNT="$((${COUNT} * ${ITERATION}))"
echo "Benchmark ${COUNT}ops, ${ITERATION}it on ${TARGET_HOST}:${TARGET_PORT}"

source "tqdm.sh"
TQDM="$(get_tqdm_command "${TOTAL_COUNT}")"

START="$(date +"%s.%N")"
for I in $(seq "${ITERATION}"); do
  cat "${INPUT}" \
    | nc -q0 "${TARGET_HOST}" "${TARGET_PORT}" \
    | grep -v '^\$[^-]'
done | ${TQDM} > /dev/null

END="$(date +"%s.%N")"
ELAPSED="$(echo "(${END} - ${START})" | bc -l)"
OP_PER_US="$(echo "(${ELAPSED} * 1000000) / (${TOTAL_COUNT})" | bc -l)"
echo "Elapsed tot: ${ELAPSED}s"
echo "Elapsed 1op: ${OP_PER_US}us"

