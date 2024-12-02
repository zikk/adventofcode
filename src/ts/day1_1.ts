import path from "node:path";

const IS_PROD = true;
const TEST_INPUT_PATH = path.join(__dirname, "../../inputs/day1.test.in");
const PROD_INPUT_PATH = path.join(__dirname, "../../inputs/day1.prod.in");
const INPUT_PATH = IS_PROD ? PROD_INPUT_PATH : TEST_INPUT_PATH;

const file = Bun.file(INPUT_PATH);
const text = await file.text();

const a: number[] = [];
const b: number[] = [];

text.trim().split("\n").forEach((line) => {
  const [va, vb] = line.split("   ");
  a.push(Number.parseInt(va, 10));
  b.push(Number.parseInt(vb, 10));
});

a.sort((a, b) => a - b);
b.sort((a, b) => a - b);

let result = 0;

for (let i = 0; i < a.length; i += 1) {
  result += Math.abs(a[i] - b[i]);
}

console.log("Result :", result);
