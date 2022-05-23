import { serve } from "https://deno.land/std@0.140.0/http/server.ts";
import init, {build as ramCompile} from "https://cloudshare.download/ram-lang/compiler/0.1.0/wasm.js"

await init()

serve(async (req) => {
  const url = new URL(req.url);

  if (url.pathname !== "/run") {
    return new Response("", { status: 404 });
  }

  const inputCode = ramCompile(url.searchParams.get("code") || "");

  const p = Deno.run({
    cmd: [
      "deno",
      "run",
      "./run.ts",
    ],
    stdin: "piped",
    stdout: "piped",
  });
  p.stdin.write(new TextEncoder().encode(inputCode));
  p.stdin.close();
  const { success } = await p.status();
  if (!success) {
    return new Response("", { status: 500 });
  }
  const rawOutput = await p.output();

  const resp = new TextDecoder().decode(rawOutput);

  return new Response(resp);
});
