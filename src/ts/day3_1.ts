import path from "node:path";

const IS_PROD = true;
const TEST_INPUT_PATH = path.join(__dirname, "../../inputs/day3.test.in");
const PROD_INPUT_PATH = path.join(__dirname, "../../inputs/day3.prod.in");
const INPUT_PATH = IS_PROD ? PROD_INPUT_PATH : TEST_INPUT_PATH;

const file = Bun.file(INPUT_PATH);
const text = await file.text();

const RE = /mul\(\d+,\d+\)/g;

let result = 0;

[...text.matchAll(RE)].forEach((x) => {
  const exp = x[0];
  const r = exp.split("(");
  const [astr, bstr] = r[1].split(",");
  const a = Number.parseInt(astr, 10);
  const b = Number.parseInt(bstr, 10);

  result += a * b;
})

console.log("Result :", result);
