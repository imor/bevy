use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Query, SystemState},
    bundle::Bundle,
    world::World,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{prelude::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

criterion_group!(
    benches,
    world_entity,
    world_get,
    world_query_get,
    world_query_iter,
    world_query_for_each,
    query_get_component,
    query_get,
);
criterion_main!(benches);

#[derive(Component, Default)]
#[component(storage = "Table")]
struct Table(f32);
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
struct Sparse(f32);
#[derive(Component, Default)]
#[component(storage = "Table")]
struct WideTable<const X: usize>(f32);
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
struct WideSparse<const X: usize>(f32);

const RANGE: std::ops::Range<u32> = 5..6;

fn deterministic_rand() -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(42)
}

fn setup<T: Component + Default>(entity_count: u32) -> World {
    let mut world = World::default();
    world.spawn_batch((0..entity_count).map(|_| (T::default(),)));
    black_box(world)
}

fn setup_wide<T: Bundle + Default>(entity_count: u32) -> World {
    let mut world = World::default();
    world.spawn_batch((0..entity_count).map(|_| T::default()));
    black_box(world)
}

fn world_entity(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("world_entity");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities", entity_count), |bencher| {
            let world = setup::<Table>(entity_count);

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    black_box(world.entity(entity));
                }
            });
        });
    }

    group.finish();
}

fn world_get(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("world_get");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities_table", entity_count), |bencher| {
            let world = setup::<Table>(entity_count);

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    assert!(world.get::<Table>(entity).is_some());
                }
            });
        });
        group.bench_function(format!("{}_entities_sparse", entity_count), |bencher| {
            let world = setup::<Sparse>(entity_count);

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    assert!(world.get::<Sparse>(entity).is_some());
                }
            });
        });
    }

    group.finish();
}

fn world_query_get(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("world_query_get");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities_table", entity_count), |bencher| {
            let mut world = setup::<Table>(entity_count);
            let mut query = world.query::<&Table>();

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    assert!(query.get(&world, entity).is_ok());
                }
            });
        });
        group.bench_function(format!("{}_entities_table_wide", entity_count), |bencher| {
            let mut world = setup_wide::<(
                WideTable<0>,
                WideTable<1>,
                WideTable<2>,
                WideTable<3>,
                WideTable<4>,
                WideTable<5>,
            )>(entity_count);
            let mut query = world.query::<(
                &WideTable<0>,
                &WideTable<1>,
                &WideTable<2>,
                &WideTable<3>,
                &WideTable<4>,
                &WideTable<5>,
            )>();

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    assert!(query.get(&world, entity).is_ok());
                }
            });
        });
        group.bench_function(format!("{}_entities_sparse", entity_count), |bencher| {
            let mut world = setup::<Sparse>(entity_count);
            let mut query = world.query::<&Sparse>();

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    assert!(query.get(&world, entity).is_ok());
                }
            });
        });
        group.bench_function(format!("{}_entities_sparse_wide", entity_count), |bencher| {
            let mut world = setup_wide::<(
                WideSparse<0>,
                WideSparse<1>,
                WideSparse<2>,
                WideSparse<3>,
                WideSparse<4>,
                WideSparse<5>,
            )>(entity_count);
            let mut query = world.query::<(
                &WideSparse<0>,
                &WideSparse<1>,
                &WideSparse<2>,
                &WideSparse<3>,
                &WideSparse<4>,
                &WideSparse<5>,
            )>();

            bencher.iter(|| {
                for i in 0..entity_count {
                    let entity = Entity::from_raw(i);
                    assert!(query.get(&world, entity).is_ok());
                }
            });
        });
    }

    group.finish();
}

fn world_query_iter(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("world_query_iter");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities_table", entity_count), |bencher| {
            let mut world = setup::<Table>(entity_count);
            let mut query = world.query::<&Table>();

            bencher.iter(|| {
                let mut count = 0;
                for comp in query.iter(&world) {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                }
                assert_eq!(black_box(count), entity_count);
            });
        });
        group.bench_function(format!("{}_entities_sparse", entity_count), |bencher| {
            let mut world = setup::<Sparse>(entity_count);
            let mut query = world.query::<&Sparse>();

            bencher.iter(|| {
                let mut count = 0;
                for comp in query.iter(&world) {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                }
                assert_eq!(black_box(count), entity_count);
            });
        });
    }

    group.finish();
}

fn world_query_for_each(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("world_query_for_each");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities_table", entity_count), |bencher| {
            let mut world = setup::<Table>(entity_count);
            let mut query = world.query::<&Table>();

            bencher.iter(|| {
                let mut count = 0;
                query.for_each(&world, |comp| {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                });
                assert_eq!(black_box(count), entity_count);
            });
        });
        group.bench_function(format!("{}_entities_sparse", entity_count), |bencher| {
            let mut world = setup::<Sparse>(entity_count);
            let mut query = world.query::<&Sparse>();

            bencher.iter(|| {
                let mut count = 0;
                query.for_each(&world, |comp| {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                });
                assert_eq!(black_box(count), entity_count);
            });
        });
    }

    group.finish();
}

fn query_get_component(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query_get_component");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities_table", entity_count), |bencher| {
            let mut world = World::default();
            let mut entities: Vec<_> = world
                .spawn_batch((0..entity_count).map(|_| (Table::default(),)))
                .collect();
            entities.shuffle(&mut deterministic_rand());
            let mut query = SystemState::<Query<&Table>>::new(&mut world);
            let query = query.get(&world);

            bencher.iter(|| {
                let mut count = 0;
                for comp in entities
                    .iter()
                    .flat_map(|&e| query.get_component::<Table>(e))
                {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                }
                assert_eq!(black_box(count), entity_count);
            });
        });
        group.bench_function(format!("{}_entities_sparse", entity_count), |bencher| {
            let mut world = World::default();
            let mut entities: Vec<_> = world
                .spawn_batch((0..entity_count).map(|_| (Sparse::default(),)))
                .collect();
            entities.shuffle(&mut deterministic_rand());
            let mut query = SystemState::<Query<&Sparse>>::new(&mut world);
            let query = query.get(&world);

            bencher.iter(|| {
                let mut count = 0;
                for comp in entities
                    .iter()
                    .flat_map(|&e| query.get_component::<Sparse>(e))
                {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                }
                assert_eq!(black_box(count), entity_count);
            });
        });
    }

    group.finish();
}

fn query_get(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query_get");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    for entity_count in RANGE.map(|i| i * 10_000) {
        group.bench_function(format!("{}_entities_table", entity_count), |bencher| {
            let mut world = World::default();
            let mut entities: Vec<_> = world
                .spawn_batch((0..entity_count).map(|_| (Table::default(),)))
                .collect();
            entities.shuffle(&mut deterministic_rand());
            let mut query = SystemState::<Query<&Table>>::new(&mut world);
            let query = query.get(&world);

            bencher.iter(|| {
                let mut count = 0;
                for comp in entities.iter().flat_map(|&e| query.get(e)) {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                }
                assert_eq!(black_box(count), entity_count);
            });
        });
        group.bench_function(format!("{}_entities_sparse", entity_count), |bencher| {
            let mut world = World::default();
            let mut entities: Vec<_> = world
                .spawn_batch((0..entity_count).map(|_| (Sparse::default(),)))
                .collect();
            entities.shuffle(&mut deterministic_rand());
            let mut query = SystemState::<Query<&Sparse>>::new(&mut world);
            let query = query.get(&world);

            bencher.iter(|| {
                let mut count = 0;
                for comp in entities.iter().flat_map(|&e| query.get(e)) {
                    black_box(comp);
                    count += 1;
                    black_box(count);
                }
                assert_eq!(black_box(count), entity_count);
            });
        });
    }

    group.finish();
}
