extern crate centaurus;
#[macro_use]
extern crate criterion;

use centaurus::SceneBuilder;
use criterion::Criterion;

fn render_over_image_size(width: usize, height: usize) {
    let scene = SceneBuilder::new()
        .with_width(width)
        .with_height(height)
        .build();
    scene.render();
}

fn render_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "render over image size",
        |b, &&size| b.iter(|| render_over_image_size(size, size)),
        &[8, 16, 32, 64, 128],
    );
}

criterion_group!(benches, render_benchmark);
criterion_main!(benches);
