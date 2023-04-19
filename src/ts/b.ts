const input = await Deno.readTextFile('./input.prod');

const bridge = new Map();
const knotsPositions = [
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
  [0, 0],
]

bridge.set('0,0', true);

function moveKnot(knotIndex: number) {
  const currentKnotPos = knotsPositions[knotIndex];
  const previousKnotPos = knotsPositions[knotIndex - 1];
  if (Math.abs(previousKnotPos[0] - currentKnotPos[0]) <= 1 && Math.abs(previousKnotPos[1] - currentKnotPos[1]) <= 1) {
    return;
  }

  const xDiff = previousKnotPos[0] - currentKnotPos[0];
  const yDiff = previousKnotPos[1] - currentKnotPos[1];

  if (currentKnotPos[0] === previousKnotPos[0] && Math.abs(yDiff) > 1) {
    currentKnotPos[1] += Math.sign(yDiff);
  } else if (currentKnotPos[1] === previousKnotPos[1] && Math.abs(xDiff) > 1) {
    currentKnotPos[0] += Math.sign(xDiff);
  } else {
    currentKnotPos[0] += Math.sign(xDiff);
    currentKnotPos[1] += Math.sign(yDiff);
  }

  if (knotIndex === 9) {
    bridge.set(`${currentKnotPos[0]},${currentKnotPos[1]}`, true);
  }

  return;
}

function moveKnots() {
  for(let i = 1; i < 10; i += 1) {
    moveKnot(i);
  }
}

input.split('\n').forEach((line) => {
  const [dir, stepsStr] = line.split(' ');
  const steps = Number.parseInt(stepsStr, 10);
  const headPos = knotsPositions[0];

  switch(dir) {
    case 'R':
      for(let i = 0; i < steps; i += 1) {
        headPos[1] += 1;
        moveKnots();
      }
      break;

    case 'L':
      for(let i = 0; i < steps; i += 1) {
        headPos[1] -= 1;
        moveKnots();
      }
      break;

    case 'U':
      for(let i = 0; i < steps; i += 1) {
        headPos[0] -= 1;
        moveKnots();
      }
      break;

    case 'D':
      for(let i = 0; i < steps; i += 1) {
        headPos[0] += 1;
        moveKnots();
      }
      break;
  }
});

let result = 0;

for(const v of bridge.values()) {
  if (v) {
    result += 1;
  }
}

console.log("🚀 ~ file: a.ts:95 ~ result", result)
