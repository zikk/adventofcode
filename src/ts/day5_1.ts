const input = await Deno.readTextFile('./input.prod');

const numberOfStacks = (input.split('\n')[0].length + 1) / 4;
let stacks = new Array(numberOfStacks).fill([]);
let shouldSkip = false;
let indicesLineIndex = 0;

input.split('\n').forEach((line, lineIndex) => {
  if (shouldSkip) return;

  for (let i = 0; i < numberOfStacks * 4 - 1; i += 4) {
    const crateIndex = i / 4;
    const crate = line.slice(i, i + 4)[1];

    if (crate !== ' ' && !Number.isInteger(Number.parseInt(crate, 10))) {
      stacks[crateIndex] = [...stacks[crateIndex], crate];
    }

    if (Number.isInteger(Number.parseInt(crate, 10))) {
      shouldSkip = true;
    }
  }

  indicesLineIndex = lineIndex;
});

stacks = stacks.map((stack) => stack.reverse());

const commands = input.split('\n').slice(indicesLineIndex + 2);


commands.forEach((command) => {
  const digits = command.match(/\d+/g)?.map((digit) => Number.parseInt(digit, 10)) ?? [];

  for(let i = 0; i < digits[0]; i += 1) {
    const crate = stacks[digits[1] - 1].pop();
    stacks[digits[2] - 1].push(crate);
  }
})

const result = stacks.map((stack) => stack.pop()).join('')
console.log(result);
