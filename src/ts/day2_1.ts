import path from "node:path";

const IS_PROD = true;
const TEST_INPUT_PATH = path.join(__dirname, "../../inputs/day2.test.in");
const PROD_INPUT_PATH = path.join(__dirname, "../../inputs/day2.prod.in");
const INPUT_PATH = IS_PROD ? PROD_INPUT_PATH : TEST_INPUT_PATH;

const file = Bun.file(INPUT_PATH);
const text = await file.text();

function isLevelSafe(values: number[]) {
  let isInc = false;
  let isSafe = true;

  for (let i = 0; i < values.length; i += 1) {
    if (!isSafe) continue;

    const a = values[i];
    const b = values[i + 1];
    let diff = Math.abs(a - b);

    if (i === 0) isInc = a < b;
    if (diff < 1 || diff > 3) isSafe = false;

    if (i > 0) {
      if (isInc && a > b || !isInc && a < b) isSafe = false;
    }
  }

  return isSafe;
}

const levels = text.split("\n").map((line) => {
  return line.split(" ").map((v) => Number.parseInt(v, 10));
});

let result = 0;

for (let i = 0; i < levels.length - 1; i += 1) {
  const level = levels[i];
  if (isLevelSafe(level)) result += 1;
}

console.log("Result :", result);
