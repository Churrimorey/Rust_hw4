use std::{
    collections::VecDeque,
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use async_std::task::spawn;

use scoped_tls::scoped_thread_local;
use signal::Signal;
use task::Task;

mod signal;
mod task;

scoped_thread_local!(static SIGNAL:Arc<Signal>);
scoped_thread_local!(static RUNNABLE:Mutex<VecDeque<Arc<Task>>>);

fn main() {
    block_on(demo());
}

async fn demo() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    // std::thread::spawn(move || {
    //     std::thread::sleep(Duration::from_secs(20));
    //     let _ = tx.send_blocking(());
    // });
    spawn(demo2(tx));
    println!("hello world!");
    let _ = rx.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("hello world2!");
    let _ = tx.send(()).await;
}

// fn dummy_waker() -> Waker {
//     static DATA: () = ();
//     unsafe { Waker::from_raw(RawWaker::new(&DATA, &VTABLE)) }
// }

// const VTABLE: RawWakerVTable =
//     RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);

// unsafe fn vtable_clone(_p: *const ()) -> RawWaker {
//     RawWaker::new(_p, &VTABLE)
// }

// unsafe fn vtable_wake(_p: *const ()) {}

// unsafe fn vtable_wake_by_ref(_p: *const ()) {}

// unsafe fn vtable_drop(_p: *const ()) {}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut main_fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());

    let mut main_cx = Context::from_waker(&waker);
    let runnable = Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || loop {
            if let Poll::Ready(output) = main_fut.as_mut().poll(&mut main_cx) {
                return output;
            }
            while let Some(task) = runnable.lock().unwrap().pop_back() {
                let waker = Waker::from(task.clone());
                let mut cx = Context::from_waker(&waker);
                let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
            }
            signal.wait();
        })
    })
}
