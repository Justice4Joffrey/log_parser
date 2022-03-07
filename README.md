Log Parser
==========

### TLDR

Build with

```
cargo build --release --features=default
```

Optionally generate some dummy data

```
python3 generate_bench_data.py --n 1000000 example_data.log
```

Run the parser. There are various options available with the `help` command. Both the sync and async versions are
bundled.

```
./target/release/log_parser --json async example_data.log
```

### Task

There are a number of factors which make writing a general, performant solution to this tricky.

- Log files can be expected to be large =>
    - We shouldn't read entire file to memory at once
    - This complicates the lifetime management. We can't expect bytes to live for `'static`.

I'm going to start by implementing a very simple single-threaded parser using a `BufReader`. I'll then benchmark and
test some performance optimizations.

The first question would be 'What are the performance implications of using `regex` over `serde`?'.

In terms of macro-optimizations, we should try to utilize multi-threading. It's almost certain that this process will be
IO bound. We should therefore focus on strategies which improve IO throughput. This is likely argument for async I/O
over parallelism, as it enables us to work _while_ we're waiting for more data to arrive.

You can re-run these benchmarks by generating some dummy data (command above) and running

```
cargo bench
```

### Sync Results

- Our first-pass `serde_json` parser completes our 1M line file in 261.58 ms (median).

- Our second pass `regex` parser had comparable performance 269.80 ms.

- Our third pass string search was a vast improvement coming in with a median of 100.45 ms.

- What looked like an optimization of `StringParser`'s `find_subsequence`, `find_char`, was inexplicably slower. I'm
  guessing this is down to CPU-caching. I left the `CharParser` in for comparison.

### Macro-optimization

After getting the naive version out the way, I was interested in attempting to implement a more performant version.

The task lends itself to a map-reduce style approach.

##### The Algorithm

- One `tokio::task` reads the file in batches of newlines (I had to implement this manually, strangely) and spawns a
  task per batch.
- Each task owns its own buffer, which means it can produce a reference type `LogLineMetadata<'a>` with a slice on that
  buffer to reduce the amount of allocations.
- Just like the synchronous version, `LogLineMetadata<'a>`s can be accumulated in to a `Summary`, which is the final
  result type.
- Another task listens on an `mpsc::channel` for the aggregated `Summary`s of the parser tasks, 'reducing' these in to a
  single `Summary` through `Summary::combine`, which is returned at the end. This felt like it would almost certainly be
  an improvement over using a lock, but that's something which could be benchmarked.

We made a very nervy start with our initial batch value (coming in 2x slower than the sync version), but after a bit of
tuning with the batch size, we were able to get to a median value of 64.533 ms, around 1.8x faster. It looks like
performance is convex in relation to batch size. It improves towards values of ~1MB and then gradually degrades. With a
very small batch size, the algorithm is very slow.

Unfortunately, we don't track parse failure lines with the batched version. Tracking the actual line-numbers when
operating over batches felt like an incredibly difficult thing to do.

A future optimization might involve the reader cycling through a set of pre-allocated buffers, which it can pass to the
parser sub-tasks (like a ring-buffer).

### Future work

Things I didn't get around to implementing that would be nice:

- decompressing compressed log files
- automatically grow the sync version's buffer if a line is too long
- might want to do some feature gating to avoid bundling unwanted dependencies (e.g. serde, regex, tokio).