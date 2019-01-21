const js = import("./pkg/wasm_tutorial");

js.then(js => {
  console.log(js.fib(10))
});
