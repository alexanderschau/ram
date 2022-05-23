const readStdin = async () => {
  const bytes: number[] = [];

  while (true) {
    const buffer = new Uint8Array(1);
    const readStatus = await Deno.stdin.read(buffer);

    // Found EOL
    if (readStatus === null || readStatus === 0) {
      break;
    }

    bytes.push(buffer[0]);
  }

  return new TextDecoder().decode(Uint8Array.from(bytes));
};

const inputCode = await readStdin();
eval(inputCode)