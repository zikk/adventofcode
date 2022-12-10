const input = await Deno.readTextFile('./input.prod');

const increments: number[] = [];

input.split('\n').forEach((line) => {
  const [command, value] = line.split(' ');
  increments.push(0);

  if (command === 'addx') {
    increments.push(parseInt(value));
  }
});

let totalsSum = 0;

for(let i = 20; i < increments.length; i += 40) {
  let total = 1;

  for(let j = 0; j < i - 1; j += 1) {
    total += increments[j] || 0;
  }

  totalsSum += total * i;
}

console.log("🚀 ~ file: a.ts:26 ~ totalsSum", totalsSum)
