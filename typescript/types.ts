type MyPartial<T> = {
  [P in keyof T]?: T[P];
};

// 测试 MyPartial
interface Person {
  name: string;
  age: number;
}

type PartialPerson = MyPartial<Person>;
// PartialPerson 相当于 { name?: string; age?: number; }

type MyPick<T, K extends keyof T> = {
  [P in K]: T[P];
};

// 测试 MyPick
interface Person {
  name: string;
  age: number;
  address: string;
}

type PickPerson = MyPick<Person, "name" | "age">;
// PickPerson 相当于 { name: string; age: number; }

type MyExclude<T, U> = T extends U ? never : T;

// 测试 MyExclude
type T = "a" | "b" | "c";
type U = "a";

type Excluded = MyExclude<T, U>;
// Excluded 相当于 'b' | 'c'

type MyOmit<T, K extends keyof any> = Pick<T, Exclude<keyof T, K>>;

// 测试 MyOmit
interface Person {
  name: string;
  age: number;
  address: string;
}

type OmitPerson = MyOmit<Person, "address">;
// OmitPerson 相当于 { name: string; age: number; }
