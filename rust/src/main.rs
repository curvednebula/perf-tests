#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use ahash::{HashMap, HashMapExt};
use compact_str::{CompactString, ToCompactString};
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::{sync::Semaphore, task::JoinSet, time::Instant};

const TASKS_NUM: u32 = 100_000;
const ITEMS_NUM: u32 = 10_000;

struct SomeData {
    name: CompactString,
    num: u32,
}

fn test_task() -> Duration {
    let task_start = Instant::now();
    let mut map = HashMap::with_capacity(ITEMS_NUM.try_into().unwrap());
    let mut _sum: u64 = 0;

    for j in 0..ITEMS_NUM {
        let name = j.to_compact_string();

        map.insert(
            name.clone(),
            SomeData {
                name: name.clone(),
                num: j,
            },
        );

        let val = map.get(&name);
        if let Some(value) = val {
            if value.name == name {
                _sum += value.num as u64;
            }
        }
    }
    task_start.elapsed()
}

async fn run_test(task_mult: NonZeroUsize) {
    let start = Instant::now();
    let mut join_set = JoinSet::new();
    let sem = Arc::new(Semaphore::new(num_cpus::get() * task_mult.get()));

    let test_task_fn = async |sem: Arc<Semaphore>| {
        let _permit = sem.acquire_owned().await;
        test_task()
    };

    for _ in 0..TASKS_NUM {
        join_set.spawn(test_task_fn(sem.clone()));
    }

    let mut num_results = 0;
    let mut all_tasks_time = Duration::ZERO;
    let mut min_time = Duration::MAX;
    let mut max_time = Duration::ZERO;

    while let Some(Ok(task_time)) = join_set.join_next().await {
        all_tasks_time += task_time;
        if min_time > task_time {
            min_time = task_time;
        }
        if max_time < task_time {
            max_time = task_time;
        }
        num_results += 1;
    }

    assert!(num_results == TASKS_NUM);

    let total_duration = start.elapsed();
    let avg_time = all_tasks_time / num_results;

    println!(
        "- finished in {:?}, task avg {:?}, min {:?}, max {:?}",
        total_duration, avg_time, min_time, max_time
    );
}

#[tokio::main]
async fn main() {
    run_test(NonZeroUsize::MIN).await;
    run_test(NonZeroUsize::MIN).await;
    run_test(NonZeroUsize::new(5).unwrap()).await;
    run_test(NonZeroUsize::new(10).unwrap()).await;
}
