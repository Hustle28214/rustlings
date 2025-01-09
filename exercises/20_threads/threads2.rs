// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // 使用 `Arc` 和 `Mutex` 包裹 `JobStatus`，以实现线程安全的共享可变状态
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        // 克隆 `Arc`，以便每个线程可以共享所有权
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // 获取 `Mutex` 的锁，以安全地修改共享数据
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终的 `jobs_done` 值
    let status = status.lock().unwrap();
    println!("Jobs done: {}", status.jobs_done);
}