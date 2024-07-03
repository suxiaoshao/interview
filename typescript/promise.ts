type Executor<T> = (
  resole: (data: T) => void,
  reject: (err: any) => void,
) => void;
enum PromiseType {
  PENDING,
  FULFILLED,
  REJECTED,
}

export type Enum<
  Name extends string | number,
  Value = undefined,
> = Value extends undefined
  ? { tag: Name; value?: Value }
  : {
      tag: Name;
      value: Value;
    };

type PromiseState<T> =
  | Enum<PromiseType.PENDING>
  | Enum<PromiseType.FULFILLED, T>
  | Enum<PromiseType.REJECTED, any>;
type ResolveFunction<T> = (value: T) => void;
type RejectFunction = (reason?: any) => void;
type ThenFunction<T, TResult> = (value: T) => TResult;
type CatchFunction<TResult> = (reason?: any) => TResult;

class CustomPromise<T> {
  executor: Executor<T>;
  state: PromiseState<T>;
  private thenCallbacks: Array<ResolveFunction<any>> = [];
  private catchCallbacks: Array<RejectFunction> = [];
  constructor(executor: Executor<T>) {
    this.executor = executor;
    this.state = {
      tag: PromiseType.PENDING,
    };
    try {
      executor(this.resolve.bind(this), this.reject.bind(this));
    } catch (err) {
      this.reject(err);
    }
  }
  private resolve(value: T) {
    if (this.state.tag !== PromiseType.PENDING) return;
    const state: PromiseState<T> = {
      tag: PromiseType.FULFILLED,
      value: value,
    } as PromiseState<T>;
    this.state = state;
    this.runThenCallbacks();
  }

  private reject(reason: any) {
    if (this.state.tag !== PromiseType.PENDING) return;
    const state = {
      tag: PromiseType.REJECTED,
      value: reason,
    } as PromiseState<T>;
    this.state = state;
    this.runCatchCallbacks();
  }
  private runThenCallbacks() {
    switch (this.state.tag) {
      case PromiseType.FULFILLED:
        this.thenCallbacks.forEach((callback) => {
          callback(this.state.value);
        });
        this.thenCallbacks = [];
    }
  }

  private runCatchCallbacks() {
    switch (this.state.tag) {
      case PromiseType.REJECTED:
        this.catchCallbacks.forEach((callback) => {
          callback(this.state.value);
        });
        this.catchCallbacks = [];
    }
  }
  then<TResult = T>(
    onFulfilled?: ThenFunction<T, TResult>,
    onRejected?: CatchFunction<TResult>,
  ): CustomPromise<TResult> {
    return new CustomPromise<TResult>((resolve, reject) => {
      this.thenCallbacks.push((value: T) => {
        if (onFulfilled) {
          try {
            resolve(onFulfilled(value));
          } catch (err) {
            reject(err);
          }
        } else {
          resolve(value as any as TResult);
        }
      });

      this.catchCallbacks.push((reason: any) => {
        if (onRejected) {
          try {
            resolve(onRejected(reason));
          } catch (err) {
            reject(err);
          }
        } else {
          reject(reason);
        }
      });

      this.runThenCallbacks();
      this.runCatchCallbacks();
    });
  }
  catch<TResult = never>(
    onRejected?: CatchFunction<TResult>,
  ): CustomPromise<T | TResult> {
    return this.then(undefined, onRejected);
  }
}

// Example usage:
const p = new CustomPromise<number>((resolve, reject) => {
  setTimeout(() => {
    resolve(42);
  }, 1000);
});

p.then((result) => {
  console.log(result); // 42
  return result + 1;
})
  .then((result) => {
    console.log(result); // 43
    return result + 1;
  })
  .then((result) => {
    console.log(result); // 43
  })
  .catch((error) => {
    console.error(error);
  });
