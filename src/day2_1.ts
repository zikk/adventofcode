const IS_PROD = true;
const TEST_INPUT_PATH = "./inputs/day2.test.in";
const PROD_INPUT_PATH = "./inputs/day2.prod.in";
const INPUT_PATH = IS_PROD ? PROD_INPUT_PATH : TEST_INPUT_PATH;

const file = Bun.file(INPUT_PATH);
const text = await file.text();
