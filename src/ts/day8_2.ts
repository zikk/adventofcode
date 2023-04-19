const input = await Deno.readTextFile('./input.prod');

const trees = input.split('\n').map((l) => l.split('').map(Number));
const width = trees[0].length;
const height = trees.length;

let highestScenicScore = 0;

for(let y = 0; y < width; y += 1) {
  for(let x = 0; x < height; x += 1) {
    if (
      y === 0
      || x === 0
      || y === height - 1
      || x === width - 1
    ) {
      continue;
    }
    const topIndices = Array.from(Array(y).keys(), v => v).reverse();
    const leftIndices = Array.from(Array(x).keys(), v => v).reverse();
    const rightIndices = Array.from(Array(width - x - 1).keys(), v => v + x + 1);
    const bottomIndices = Array.from(Array(height - y - 1).keys(), v => v + y + 1);

    let seenOnTop = 0;
    let seeOnLeft = 0;
    let seenOnRight = 0;
    let seenOnBotton = 0;

    topIndices.some((i) => {
      seenOnTop += 1;
      return trees[i][x] >= trees[y][x];
    });

    leftIndices.some((i) => {
      seeOnLeft += 1;
      return trees[y][i] >= trees[y][x];
    });

    rightIndices.some((i) => {
      seenOnRight += 1;
      return trees[y][i] >= trees[y][x];
    });

    bottomIndices.some((i) => {
      seenOnBotton += 1;
      return trees[i][x] >= trees[y][x];
    });

    const scenicScore = seenOnTop * seeOnLeft * seenOnRight * seenOnBotton;
    highestScenicScore = Math.max(highestScenicScore, scenicScore);
  }
}

console.log("🚀 ~ file: a.ts:74 ~ highestScenicScore", highestScenicScore)
