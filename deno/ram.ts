// start helpers
const writeString = (
  memory: WebAssembly.Memory,
  str: string,
  offset: number,
) => {
  const strBuf = new TextEncoder().encode(str);
  const outBuf = new Uint8Array(memory.buffer, offset, strBuf.length);
  for (let i = 0; i < strBuf.length; i++) {
    outBuf[i] = strBuf[i];
  }
};
function copyCStr(wasmMemory: WebAssembly.Memory, ptr: number) {
  const collectCString = function* () {
    const memory = new Uint8Array(wasmMemory.buffer);
    while (memory[ptr] !== 0) {
      if (memory[ptr] === undefined) throw new Error("Tried to read undef mem");
      yield memory[ptr];
      ptr += 1;
    }
  };

  const buffer_as_u8 = new Uint8Array(collectCString());
  const utf8Decoder = new TextDecoder("UTF-8");
  const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
  return buffer_as_utf8;
}

// end helpers

const wasmCode = await Deno.readFile(
  "../compiler/target/wasm32-unknown-unknown/release/ram.wasm",
);

const wasmModule = new WebAssembly.Module(wasmCode);

const wasmInstance = new WebAssembly.Instance(wasmModule, {
  env: {
    print: (start: number, len: number) => {
      const m = new Uint8Array(wasmInstance.exports.memory.buffer);
      console.log("ARG", new TextDecoder().decode(m.slice(start, start + len)));
    },
  },
}) as WebAssembly.Instance & {
  exports: {
    memory: WebAssembly.Memory;
    compile: (n: number) => number;
    dealloc: (n: number) => void;
    alloc: () => number;
  };
};

export const compile = (code: string): string => {
  const pointer = wasmInstance.exports.alloc();
  writeString(wasmInstance.exports.memory, code, pointer);
  const s = wasmInstance.exports.compile(pointer);
  return copyCStr(wasmInstance.exports.memory, s);
};
