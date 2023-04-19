const input = await Deno.readTextFile('./input.prod');

const shapes = {
  rock: {
    A: 1,
  },
  paper: {
    B: 2,
  },
  scisor: {
    C: 3,
  },
}

const results = {
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
}

function play(a: "A" | "B" | "C", b: "X" | "Y" | "Z") {
  const shapeA = Object.keys(shapes).find(shape => shapes[shape][a]);

  if (b === 'X') {
    const shapeB = Object.keys(results[shapeA!]).find((shape) => results[shapeA!][shape] === 'win');
    return Math.max(Object.values(shapes[shapeB])) + 0
  }

  if (b === 'Y') {
    const shapeB = Object.keys(results[shapeA!]).find((shape) => results[shapeA!][shape] === 'draw');
    return Math.max(Object.values(shapes[shapeB])) + 3
  }

  if (b === 'Z') {
    const shapeB = Object.keys(results[shapeA!]).find((shape) => results[shapeA!][shape] === 'lose');
    return Math.max(Object.values(shapes[shapeB])) + 6
  }
}

const result = input.split('\n').reduce((acc, line) => {
  const [a, b] = line.split(' ');
  return acc + play(a, b);
}, 0);

console.log(result);