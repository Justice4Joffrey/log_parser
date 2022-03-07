use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log_parser::{
    AsyncBufReaderSummarizer, BufReaderSummarizer, CharParser, JsonParser, RegexParser,
    StringParser, Summarizer,
};

use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("json_parser", |b| {
        b.iter(|| {
            BufReaderSummarizer::default()
                .summarize::<JsonParser>(black_box("./example_data.log"))
                .unwrap()
        })
    });
    c.bench_function("regex_parser", |b| {
        b.iter(|| {
            BufReaderSummarizer::default()
                .summarize::<RegexParser>(black_box("./example_data.log"))
                .unwrap()
        })
    });
    c.bench_function("string_parser", |b| {
        b.iter(|| {
            BufReaderSummarizer::default()
                .summarize::<StringParser>(black_box("./example_data.log"))
                .unwrap()
        })
    });
    c.bench_function("char_parser", |b| {
        b.iter(|| {
            BufReaderSummarizer::default()
                .summarize::<CharParser>(black_box("./example_data.log"))
                .unwrap()
        })
    });
    c.bench_function("async_summarizer", |b| {
        b.iter(|| {
            AsyncBufReaderSummarizer::default()
                .summarize::<StringParser>(black_box("./example_data.log"))
                .unwrap()
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20).measurement_time(Duration::from_secs(20));
    targets = criterion_benchmark
);
criterion_main!(benches);
