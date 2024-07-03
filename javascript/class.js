// 一个构造函数
function Box(value) {
  this.value = value;
}

// 使用 Box() 构造函数创建的所有盒子都将具有的属性
Box.prototype.getValue = function () {
  return this.value;
};

const boxes = [new Box(1), new Box(2), new Box(3)];
console.log(boxes[0].__proto__);

function Base() {
  this.name = "base";
  console.log("base Constructor");
}
function Derived() {
  Base.apply(this);
  this.data = "Derived";
  console.log("Derived Constructor");
}
// 将 `Derived.prototype` 的 `[[Prototype]]`
// 设置为 `Base.prototype`
Object.setPrototypeOf(Derived.prototype, Base.prototype);

const obj = new Derived();
console.log(obj);

function doSomething() {}
console.log(
  doSomething.prototype,
  Object.getPrototypeOf(doSomething) === Function.prototype,
);
// 你如何声明函数并不重要；
// JavaScript 中的函数总有一个默认的
// 原型属性——有一个例外：
// 箭头函数没有默认的原型属性：
const doSomethingFromArrowFunction = () => {};
console.log(doSomethingFromArrowFunction.prototype);

console.log(
  Object.getPrototypeOf({}),
  111,
  Object.getPrototypeOf(Object.create(null)),
);
