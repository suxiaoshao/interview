Function.prototype.myBind = function (thisArgs, ...args) {
  const fnThis = this;
  return function (...fnArgs) {
    return fnThis.apply(thisArgs, [...args, ...fnArgs]);
  };
};

const modules = {
  x: 42,
  getX: function () {
    return this.x;
  },
};

const unboundGetX = modules.getX;
console.log(unboundGetX()); // The function gets invoked at the global scope
// Expected output: undefined

const boundGetX = unboundGetX.myBind(modules);
console.log(boundGetX());
// Expected output: 42
//
const boundGety = unboundGetX.bind(modules);
console.log(boundGety());
