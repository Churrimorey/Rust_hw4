# Runtime

> 因为知识能力受限，只实现了最基本未经过性能优化的runtime功能。在此阐述对代码中runtime实现的理解

## block_on

`block_on`实现`runtime`的核心函数，接口是`fn block_on<F: Future>(future: F) -> F::Output`

- 主要实现一个`loop`循环
  - 如果当前任务（主任务）完成，即`pool`返回`Pool::Ready()`，返回执行结果结果
  - 否则，通过全局任务队列`runnable`轮询辅助任务，直到就绪的任务队列为空，执行全局`SIGNAL`的`wait()`方法，直到有任务有回应

具体来说，便是先执行`demo()`，如果`demo()`堵塞并且`demo2()`已经通过`spawn()`生成，就尝试执行`demo2()`，如果仍然遇到阻塞，则等待，直到其中之一有了回应继续循环。最后，如果`demo()`执行结束，则返回函数。但是因为`demo()`需要接受`demo2()`传递的消息，因此两个`task`都会执行结束

## Signal结构

实现了`Wake trait`和`wait()`方法，实现唤醒（通过`Waker::from()`）和等待

## Task结构

- 封装了`Signal`和`Future`变量

  ```ru
  pub struct Task {
      pub future: RefCell<BoxFuture<'static, ()>>,
      signal: Arc<Signal>,
  }
  ```

- 实现了`Wake trait`，任务被唤醒时加入全局就绪任务队列等待调度