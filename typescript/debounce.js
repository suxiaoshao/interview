function debounce(func, wait) {
    var timeout;
    return function () {
        var _this = this;
        var args = [];
        for (var _i = 0; _i < arguments.length; _i++) {
            args[_i] = arguments[_i];
        }
        if (timeout) {
            clearTimeout(timeout);
        }
        timeout = setTimeout(function () {
            func.apply(_this, args);
        }, wait);
    };
}
// 使用示例
var debouncedFunction = debounce(function () {
    console.log("Function executed");
}, 300);
// 触发多次
debouncedFunction();
debouncedFunction();
debouncedFunction();
// 只有最后一次调用会在300ms后执行
