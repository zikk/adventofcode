const IS_PROD = true;
const TEST_INPUT_PATH = "./inputs/day1.test.in";
const PROD_INPUT_PATH = "./inputs/day1.prod.in";
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

let result = 0;

for (let i = 0; i < a.length; i += 1) {
  let repeats = 0;

  for (let j = 0; j < b.length; j += 1) {
    if (a[i] === b[j]) {
      repeats += 1;
    }
  }

  result += a[i] * repeats;
}

console.log(result);
