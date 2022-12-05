const input = await Deno.readTextFile('./input.test');

const numberOfStacks = (input.split('\n')[0].length + 1) / 4;
