const input = await Deno.readTextFile('./input.prod');

const bridge = new Map();
const headPos = [0, 0];
const tailPos = [0, 0];

bridge.set('0,0', true);

function moveTail() {
  if (Math.abs(headPos[0] - tailPos[0]) <= 1 && Math.abs(headPos[1] - tailPos[1]) <= 1) {
    return;
  }

  const xDiff = headPos[0] - tailPos[0];
  const yDiff = headPos[1] - tailPos[1];

  if (tailPos[0] === headPos[0] && Math.abs(yDiff) > 1) {
    tailPos[1] += Math.sign(yDiff);
  } else if (tailPos[1] === headPos[1] && Math.abs(xDiff) > 1) {
    tailPos[0] += Math.sign(xDiff);
  } else {
    tailPos[0] += Math.sign(xDiff);
    tailPos[1] += Math.sign(yDiff);
  }

  bridge.set(`${tailPos[0]},${tailPos[1]}`, true);
}

input.split('\n').forEach((line) => {
  const [dir, stepsStr] = line.split(' ');
  const steps = Number.parseInt(stepsStr, 10);

  switch(dir) {
    case 'R':
      for(let i = 0; i < steps; i += 1) {
        headPos[1] += 1;
        moveTail();
      }
      break;

    case 'L':
      for(let i = 0; i < steps; i += 1) {
        headPos[1] -= 1;
        moveTail();
      }
      break;

    case 'U':
      for(let i = 0; i < steps; i += 1) {
        headPos[0] -= 1;
        moveTail();
      }
      break;

    case 'D':
      for(let i = 0; i < steps; i += 1) {
        headPos[0] += 1;
        moveTail();
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

console.log("🚀 ~ file: a.ts:73 ~ result", result)
