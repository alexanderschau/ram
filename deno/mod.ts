import { compile } from "./ram.ts";

const runRustPlayground = async (code: string) => {
  const resp = await fetch("https://play.rust-lang.org/execute", {
    method: "post",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify({
      channel: "stable",
      mode: "debug",
      edition: "2021",
      crateType: "bin",
      tests: false,
      code: code,
      backtrace: false,
    }),
  });
  return await resp.json();
};

//console.log(await runRustPlayground(compile('📣 "hello world from ram over wasm 🐏!"')));
const compiled = compile('📣 "hello world from ram over wasm 🐏!" (➕ 4 5)');
console.log(compiled);
eval(compiled);
