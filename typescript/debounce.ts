function debounce<T extends (...args: any[]) => void>(
  func: T,
  wait: number,
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null;

  return function (...args: Parameters<T>): void {
    if (timeout) {
      clearTimeout(timeout);
    }
    timeout = setTimeout(() => {
      func.apply(this, args);
    }, wait);
  };
}

// 使用示例
const debouncedFunction = debounce(() => {
  console.log("Function executed");
}, 300);

// 触发多次
debouncedFunction();
debouncedFunction();
debouncedFunction();
// 只有最后一次调用会在300ms后执行
