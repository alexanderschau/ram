<script lang="ts">
  // import { compile, getInstance, Instance } from "./compilerBridge";
  let defaultScript = '📣 "hello world from ram over wasm 🐏!"\n📣 (➕ 4 5)';
  let output = "";
  // let instance: Instance;
  /*(async () => {
    //instance = await getInstance()
    const wasm = (await WebAssembly.instantiateStreaming(
      fetch("https://cloudshare.download/bin/ram.wasm")
    )) as WebAssembly.WebAssemblyInstantiatedSource & { instance: Instance };
    instance = wasm.instance;
  })();*/
</script>

<div class="my-4 rounded-sm border border-primary">
  <div class="flex bg-primary bg-opacity-5 px-3 py-2">
    <div class="flex-1">Ram REPL</div>
    <a
      on:click={(e) => {
        e.preventDefault();

        output = "";
        let oldLog = console.log;
        console.log = (...val) => {
          output = output + val.join(", ") + "\n";
        };
        /*eval(compile(instance, defaultScript));
        console.log = oldLog;
        console.log(compile(instance, defaultScript));*/
      }}
      class="text-text text-opacity-80 underline underline-offset-2"
      href="/"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="currentColor"
        class="my-auto inline-block h-4 align-middle"
        viewBox="0 0 16 16"
      >
        <path
          d="M10.804 8 5 4.633v6.734L10.804 8zm.792-.696a.802.802 0 0 1 0 1.392l-6.363 3.692C4.713 12.69 4 12.345 4 11.692V4.308c0-.653.713-.998 1.233-.696l6.363 3.692z"
        />
      </svg> Run Script
    </a>
  </div>
  <div class="px-3 py-2 font-mono">
    <textarea bind:value={defaultScript} class="h-36 w-full outline-none" />
  </div>
  <div class="bg-primary bg-opacity-5 px-3 py-2">
    <b>Output:</b>
    <pre class="block overflow-auto font-mono">{output}</pre>
  </div>
</div>