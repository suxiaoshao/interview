"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var PromiseType;
(function (PromiseType) {
    PromiseType[PromiseType["PENDING"] = 0] = "PENDING";
    PromiseType[PromiseType["FULFILLED"] = 1] = "FULFILLED";
    PromiseType[PromiseType["REJECTED"] = 2] = "REJECTED";
})(PromiseType || (PromiseType = {}));
var CustomPromise = /** @class */ (function () {
    function CustomPromise(executor) {
        this.thenCallbacks = [];
        this.catchCallbacks = [];
        this.executor = executor;
        this.state = {
            tag: PromiseType.PENDING,
        };
        try {
            executor(this.resolve.bind(this), this.reject.bind(this));
        }
        catch (err) {
            this.reject(err);
        }
    }
    CustomPromise.prototype.resolve = function (value) {
        if (this.state.tag !== PromiseType.PENDING)
            return;
        var state = {
            tag: PromiseType.FULFILLED,
            value: value,
        };
        this.state = state;
        this.runThenCallbacks();
    };
    CustomPromise.prototype.reject = function (reason) {
        if (this.state.tag !== PromiseType.PENDING)
            return;
        var state = {
            tag: PromiseType.REJECTED,
            value: reason,
        };
        this.state = state;
        this.runCatchCallbacks();
    };
    CustomPromise.prototype.runThenCallbacks = function () {
        var _this = this;
        switch (this.state.tag) {
            case PromiseType.FULFILLED:
                this.thenCallbacks.forEach(function (callback) {
                    callback(_this.state.value);
                });
                this.thenCallbacks = [];
        }
    };
    CustomPromise.prototype.runCatchCallbacks = function () {
        var _this = this;
        switch (this.state.tag) {
            case PromiseType.REJECTED:
                this.catchCallbacks.forEach(function (callback) {
                    callback(_this.state.value);
                });
                this.catchCallbacks = [];
        }
    };
    CustomPromise.prototype.then = function (onFulfilled, onRejected) {
        var _this = this;
        return new CustomPromise(function (resolve, reject) {
            _this.thenCallbacks.push(function (value) {
                if (onFulfilled) {
                    try {
                        resolve(onFulfilled(value));
                    }
                    catch (err) {
                        reject(err);
                    }
                }
                else {
                    resolve(value);
                }
            });
            _this.catchCallbacks.push(function (reason) {
                if (onRejected) {
                    try {
                        resolve(onRejected(reason));
                    }
                    catch (err) {
                        reject(err);
                    }
                }
                else {
                    reject(reason);
                }
            });
            _this.runThenCallbacks();
            _this.runCatchCallbacks();
        });
    };
    CustomPromise.prototype.catch = function (onRejected) {
        return this.then(undefined, onRejected);
    };
    return CustomPromise;
}());
// Example usage:
var p = new CustomPromise(function (resolve, reject) {
    setTimeout(function () {
        resolve(42);
    }, 1000);
});
p.then(function (result) {
    console.log(result); // 42
    return result + 1;
})
    .then(function (result) {
    console.log(result); // 43
    return result + 1;
})
    .then(function (result) {
    console.log(result); // 43
})
    .catch(function (error) {
    console.error(error);
});
