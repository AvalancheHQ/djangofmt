use criterion::{Criterion, black_box, criterion_group, criterion_main};
use djangofmt::args::Profile;
use djangofmt::commands::format::{FormatterConfig, format_text};

const SIMPLE_TEMPLATE: &str = include_str!("fixtures/simple.html");
const COMPLEX_TEMPLATE: &str = include_str!("fixtures/complex.html");
const WITH_STYLE_TEMPLATE: &str = include_str!("fixtures/with_style.html");

fn benchmark_simple_format(c: &mut Criterion) {
    let config = FormatterConfig::new(120, 4, None);
    let profile = Profile::Django;

    c.bench_function("format_simple_template", |b| {
        b.iter(|| {
            format_text(black_box(SIMPLE_TEMPLATE), &config, &profile)
                .expect("formatting should succeed")
        });
    });
}

fn benchmark_complex_format(c: &mut Criterion) {
    let config = FormatterConfig::new(120, 4, None);
    let profile = Profile::Django;

    c.bench_function("format_complex_template", |b| {
        b.iter(|| {
            format_text(black_box(COMPLEX_TEMPLATE), &config, &profile)
                .expect("formatting should succeed")
        });
    });
}

fn benchmark_style_format(c: &mut Criterion) {
    let config = FormatterConfig::new(120, 4, None);
    let profile = Profile::Django;

    c.bench_function("format_template_with_styles", |b| {
        b.iter(|| {
            format_text(black_box(WITH_STYLE_TEMPLATE), &config, &profile)
                .expect("formatting should succeed")
        });
    });
}

fn benchmark_multiple_line_lengths(c: &mut Criterion) {
    let profile = Profile::Django;

    let mut group = c.benchmark_group("format_line_length");

    for line_length in [80, 100, 120, 160].iter() {
        let config = FormatterConfig::new(*line_length, 4, None);
        group.bench_with_input(
            format!("complex_template_line_{}", line_length),
            line_length,
            |b, _| {
                b.iter(|| {
                    format_text(black_box(COMPLEX_TEMPLATE), &config, &profile)
                        .expect("formatting should succeed")
                });
            },
        );
    }
    group.finish();
}

fn benchmark_jinja_profile(c: &mut Criterion) {
    let config = FormatterConfig::new(120, 4, None);
    let profile = Profile::Jinja;

    c.bench_function("format_complex_jinja", |b| {
        b.iter(|| {
            format_text(black_box(COMPLEX_TEMPLATE), &config, &profile)
                .expect("formatting should succeed")
        });
    });
}

fn benchmark_custom_blocks(c: &mut Criterion) {
    let custom_blocks = Some(vec![
        "cache".to_string(),
        "compress".to_string(),
        "stage".to_string(),
    ]);
    let config = FormatterConfig::new(120, 4, custom_blocks);
    let profile = Profile::Django;

    c.bench_function("format_with_custom_blocks", |b| {
        b.iter(|| {
            format_text(black_box(COMPLEX_TEMPLATE), &config, &profile)
                .expect("formatting should succeed")
        });
    });
}

criterion_group!(
    benches,
    benchmark_simple_format,
    benchmark_complex_format,
    benchmark_style_format,
    benchmark_multiple_line_lengths,
    benchmark_jinja_profile,
    benchmark_custom_blocks
);
criterion_main!(benches);
