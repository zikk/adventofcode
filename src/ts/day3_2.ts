import path from "node:path";

const IS_PROD = true;
const TEST_INPUT_PATH = path.join(__dirname, "../../inputs/day3.test.in");
const PROD_INPUT_PATH = path.join(__dirname, "../../inputs/day3.prod.in");
const INPUT_PATH = IS_PROD ? PROD_INPUT_PATH : TEST_INPUT_PATH;

const file = Bun.file(INPUT_PATH);
const text = await file.text();

const RE = /mul\(\d+,\d+\)/g;
const DO_RE = /do()/g;
const DONT_RE = /don\'t/g;

let result = 0;

const do_indices = [...text.matchAll(DO_RE)].map((x) => {
  return x.index;
});

const dont_indices = [...text.matchAll(DONT_RE)].map((x) => {
  return x.index;
});

[...text.matchAll(RE)].forEach((x) => {
  const exp = x[0];
  const r = exp.split("(");
  const [astr, bstr] = r[1].split(",");
  const a = Number.parseInt(astr, 10);
  const b = Number.parseInt(bstr, 10);

  const last_do = do_indices.findLast((i) => {
    return i < x.index;
  });

  const last_dont = dont_indices.findLast((i) => {
    return i < x.index;
  });

  const isFirst = !last_do && !last_dont;

  if (isFirst || last_do > last_dont) {
    result += a * b;
  }
})

console.log("Result :", result);
