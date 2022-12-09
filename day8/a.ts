const input = await Deno.readTextFile('./input.prod');

const trees = input.split('\n').map((l) => l.split('').map(Number));
const width = trees[0].length;
const height = trees.length;

let totalVisible = 0;

for(let y = 0; y < width; y += 1) {
  for(let x = 0; x < height; x += 1) {
    if (
      y === 0
      || x === 0
      || y === height - 1
      || x === width - 1
    ) {
      totalVisible += 1;
      continue;
    }

    const topIndices = Array.from(Array(y).keys(), v => v);
    const leftIndices = Array.from(Array(x).keys(), v => v);
    const rightIndices = Array.from(Array(width - x - 1).keys(), v => v + x + 1);
    const bottomIndices = Array.from(Array(height - y - 1).keys(), v => v + y + 1);

    const isHiddenFromTop = topIndices.some((i) => trees[i][x] >= trees[y][x]);
    const isHiddenFromLeft = leftIndices.some((i) => trees[y][i] >= trees[y][x]);
    const isHiddenFromRight = rightIndices.some((i) => trees[y][i] >= trees[y][x]);
    const isHiddenFromBottom = bottomIndices.some((i) => trees[i][x] >= trees[y][x]);


    if (
      isHiddenFromTop
      && isHiddenFromLeft
      && isHiddenFromRight
      && isHiddenFromBottom
    ) {
      continue;
    }

    totalVisible += 1;
  }
}

console.log("🚀 ~ file: a.ts:45 ~ totalVisible", totalVisible)
