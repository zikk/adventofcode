const input = await Deno.readTextFile('./input.prod');

const shapes = {
  rock: {
    A: 1,
    X: 1,
  },
  paper: {
    B: 2,
    Y: 2,
  },
  scisor: {
    C: 3,
    Z: 3,
  },
}

function play(a: string, b: string) {
  const shapeA = Object.keys(shapes).find(shape => shapes[shape][a]);
  const shapeB = Object.keys(shapes).find(shape => shapes[shape][b]);

  const result = {
    rock: {
      rock: 'draw',
      paper: 'lose',
      scisor: 'win',
    },
    paper: {
      rock: 'win',
      paper: 'draw',
      scisor: 'lose',
    },
    scisor: {
      rock: 'lose',
      paper: 'win',
      scisor: 'draw',
    },
  }[shapeA!][shapeB];

  if (result === 'win') {
    return shapes[shapeA!][a] + 6;
  }

  if (result === 'lose') {
    return shapes[shapeA!][a];
  }

  if (result === 'draw') {
    return shapes[shapeA!][a] + 3;
  }
}

const result = input.split('\n').reduce((acc, line) => {
  const [a, b] = line.split(' ');
  return acc + play(b, a);
}, 0);

console.log(result);