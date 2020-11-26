import * as wasm from "testing-wasm";

export const jsfunc = () => {
  console.log("jsfunc called");
};

wasm.rustfunc();
