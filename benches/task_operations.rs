use criterion::{black_box, criterion_group, criterion_main, Criterion};
use repo_tasks::{Config, Task};
use std::fs;
use tempfile::TempDir;

fn benchmark_task_creation(c: &mut Criterion) {
    c.bench_function("create task", |b| {
        b.iter(|| {
            let task = Task::new(
                black_box("Test Task".to_string()),
                black_box("Medium".to_string()),
            );
            black_box(task);
        });
    });
}

fn benchmark_task_serialization(c: &mut Criterion) {
    let task = Task::new("Test Task".to_string(), "High".to_string());

    c.bench_function("serialize task to yaml", |b| {
        b.iter(|| {
            let yaml = serde_yaml::to_string(&task).unwrap();
            black_box(yaml);
        });
    });
}

fn benchmark_task_file_io(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let task = Task::new("Test Task".to_string(), "High".to_string());

    c.bench_function("write task to file", |b| {
        b.iter(|| {
            let path = temp_dir.path().join(format!("{}-test.md", task.id));
            task.to_file(&path).unwrap();
            black_box(());
        });
    });

    // Create a task file for read benchmark
    let read_path = temp_dir.path().join("read-test.md");
    task.to_file(&read_path).unwrap();

    c.bench_function("read task from file", |b| {
        b.iter(|| {
            let loaded = Task::from_file(&read_path).unwrap();
            black_box(loaded);
        });
    });
}

fn benchmark_config_operations(c: &mut Criterion) {
    c.bench_function("create default config", |b| {
        b.iter(|| {
            let config = Config::default(black_box(Some("test".to_string())));
            black_box(config);
        });
    });

    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    fs::create_dir_all(".repo-tasks").unwrap();

    let config = Config::default(Some("test".to_string()));
    config.write().unwrap();

    c.bench_function("load config from file", |b| {
        b.iter(|| {
            let loaded = Config::load().unwrap();
            black_box(loaded);
        });
    });
}

fn benchmark_slug_generation(c: &mut Criterion) {
    c.bench_function("generate slug", |b| {
        b.iter(|| {
            let slug = Task::generate_slug(black_box("Create Requirements Document"));
            black_box(slug);
        });
    });
}

criterion_group!(
    benches,
    benchmark_task_creation,
    benchmark_task_serialization,
    benchmark_task_file_io,
    benchmark_config_operations,
    benchmark_slug_generation
);
criterion_main!(benches);
