# Benchmark

## Prerequisites

- `Redis`, `NodeJS`, `nc`, `uuidgen`, `tee`, `tqdm`.

Yes, if you don't want to install `tqdm` but it can help you to show a progress bar. If you don't install it, it will be just ignored.

## Instruction

1. Run `generate.sh` to generate a list file of operations to test.
2. Run `test.sh` to send operations in that file via `nc`.

## Getting start

```bash
# Generate 1M operations with 500 random keys.
$ ./generate.sh 500 1000000
[Fri Jun 28 11:54:42 KST 2019] Generate random 500 keys
100%|████████████████████████████████████████████████████████████| 500/500 [00:00<00:00, 1909.33it/s]
[Fri Jun 28 11:54:43 KST 2019] Generate random 1000000 operations
100%|███████████████████████████████████████████████████| 1000000/1000000 [00:43<00:00, 22850.61it/s]
[Fri Jun 28 11:55:26 KST 2019] Generate a list of get/set operations
# It would generate `ops`, `ops-get` and `ops-set` files.

# Run a test with `ops` file that contains 1M random get and set operations with 10 iterations.
$ ./test.sh ops 10
Benchmark 1000000ops, 10it on localhost:6379
100%|████████████████████████████████████████████████| 10000000/10000000 [00:32<00:00, 303679.64it/s]
Elapsed tot: 32.988789912s
Elapsed 1op: 3.29887899120000000000us

# Test with this SimpleKV
$ TARGET_PORT=6378 ./test.sh ops 10
Benchmark 1000000ops, 10it on localhost:6378
100%|████████████████████████████████████████████████| 10000000/10000000 [00:42<00:00, 235822.39it/s]
Elapsed tot: 42.466095245s
Elapsed 1op: 4.24660952450000000000us
```

## Result

The elasped microseconds per 1 operation when test 1M operations with 10 iterations.

| Redis | SimpleKV |
| ----- | -------- |
| 3.3us | 4.2us    |

### Test machine

```text
Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz
MemTotal:       16287784 kB
```
