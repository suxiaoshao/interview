function throttle(func, limit) {
    var inThrottle;
    return function () {
        var args = [];
        for (var _i = 0; _i < arguments.length; _i++) {
            args[_i] = arguments[_i];
        }
        if (!inThrottle) {
            func.apply(this, args);
            inThrottle = true;
            setTimeout(function () {
                inThrottle = false;
            }, limit);
        }
    };
}
// 使用示例
var throttledFunction = throttle(function () {
    console.log("Function executed");
}, 300);
// 触发多次
throttledFunction();
throttledFunction();
throttledFunction();
// 只会在每300ms内执行一次
