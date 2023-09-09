use std::thread;
use std::time::Duration;

use async_task::Builder;
use crossbeam::channel::{unbounded, Sender};
use futures::executor;
use once_cell::sync::Lazy;

static QUEUE: Lazy<Sender<async_task::Runnable<i32>>> = Lazy::new(|| {
    let (sender, receiver) = unbounded::<async_task::Runnable<i32>>();

    for _ in 0..4 {
        let recv = receiver.clone();

        thread::spawn(|| {
            for task in recv {
                task.run();
            }
        });
    }

    sender
});

fn main() {
    let schedule = |task: async_task::Runnable<i32>| QUEUE.send(task).unwrap();

    let tasks: Vec<_> = (0..10)
        .map(|i| {
            for _ in 0..i {
                let (runnable, _task) = Builder::new().metadata(i).spawn(
                    move |_| async move {
                        for _ in 0..i {
                            thread::sleep(Duration::from_secs(2));
                            println!("Hello task {}", i);
                        }
                    },
                    schedule,
                );
                runnable.schedule();
            }
            let (runnable, task) = Builder::new().metadata(i).spawn(
                move |_| async move {
                    for _ in 0..i {
                        thread::sleep(Duration::from_secs(2));
                        println!("Hello task {}", i);
                    }
                },
                schedule,
            );
            runnable.schedule();
            task
        })
        .collect();

    // Wait for the tasks to finish.
    for task in tasks {
        executor::block_on(task);
    }
}
