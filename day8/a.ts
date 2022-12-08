const input = await Deno.readTextFile('./input.prod');

const trees = input.split('\n').map((l) => l.split('').map(Number));
const width = trees[0].length;
const height = trees.length;

let totalVisible = 0;

for(let x = 0; x < width; x += 1) {
  for(let y = 0; y < height; y += 1) {
    if (
      x === 0
      || y === 0
      || x === height - 1
      || y === width - 1
    ) {
      totalVisible += 1;
      continue;
    }

    const topIndices = Array.from(Array(x).keys(), v => v);
    const leftIndices = Array.from(Array(y).keys(), v => v);
    const rightIndices = Array.from(Array(width - y - 1).keys(), v => v + y + 1);
    const bottomIndices = Array.from(Array(height - x - 1).keys(), v => v + x + 1);

    const isHiddenFromTop = topIndices.some((i) => trees[i][y] >= trees[x][y]);
    const isHiddenFromLeft = leftIndices.some((i) => trees[x][i] >= trees[x][y]);
    const isHiddenFromRight = rightIndices.some((i) => trees[x][i] >= trees[x][y]);
    const isHiddenFromBottom = bottomIndices.some((i) => trees[i][y] >= trees[x][y]);


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
