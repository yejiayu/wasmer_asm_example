// The entry file of your WebAssembly module.
const map = new Map<i32, i32>();

export function add(a: i32, b: i32): i32 {
  map.set(a, b);

  throw new Error("test error");
  return a + b;
}
