function throttle<T extends (...args: any[]) => void>(
  func: T,
  limit: number,
): (...args: Parameters<T>) => void {
  let inThrottle: boolean;

  return function (...args: Parameters<T>): void {
    if (!inThrottle) {
      func.apply(this, args);
      inThrottle = true;
      setTimeout(() => {
        inThrottle = false;
      }, limit);
    }
  };
}

// 使用示例
const throttledFunction = throttle(() => {
  console.log("Function executed");
}, 300);

// 触发多次
throttledFunction();
throttledFunction();
throttledFunction();
// 只会在每300ms内执行一次
