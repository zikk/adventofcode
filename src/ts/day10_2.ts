const input = await Deno.readTextFile('./input.prod');

const increments: number[] = [0];
const output = [];

input.split('\n').forEach((line) => {
  const [command, value] = line.split(' ');
  increments.push(0);

  if (command === 'addx') {
    increments.push(parseInt(value));
  }
});

let x = 1;

for(let i = 0; i < increments.length; i += 40) {
  let pixelPositions = [x - 1, x, x + 1];
  const line = [];
  const instructionsSet = increments.slice(i, i + 40);

  for(let j = 0; j < instructionsSet.length; j += 1) {
    x += instructionsSet[j];
    pixelPositions = [x - 1, x, x + 1];
    if (pixelPositions.includes(j)) {
      line.push('#');
    } else {
      line.push('.');
    }
  }

  output.push(line.join(''));
}

output.pop();
console.log(output.join('\n'));
