use criterion::{criterion_group, criterion_main, Criterion};
use maplibre::{
    coords::{WorldTileCoords, ZoomLevel},
    headless::{create_headless_renderer, map::HeadlessMap},
    platform::run_multithreaded,
    style::Style,
};

fn headless_render(c: &mut Criterion) {
    c.bench_function("headless_render", |b| {
        let (mut map, tile) = run_multithreaded(async {
            let (kernel, renderer) = create_headless_renderer(1000, None).await;
            let style = Style::default();
            let map = HeadlessMap::new(style, renderer, kernel, false).unwrap();

            let tile = map
                .fetch_tile(WorldTileCoords::from((0, 0, ZoomLevel::default())))
                .await
                .expect("Failed to fetch!");

            let tile = map
                .process_tile(tile, &["water"])
                .await
                .expect("Failed to process!");

            (map, tile)
        });

        b.iter(|| {
            map.render_tile(tile.clone());
        });
    });
}

criterion_group!(name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = headless_render);
criterion_main!(benches);
