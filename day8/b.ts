const input = await Deno.readTextFile('./input.prod');

const trees = input.split('\n').map((l) => l.split('').map(Number));
const width = trees[0].length;
const height = trees.length;

let highestScenicScore = 0;

for(let x = 0; x < width; x += 1) {
  for(let y = 0; y < height; y += 1) {
    if (
      x === 0
      || y === 0
      || x === height - 1
      || y === width - 1
    ) {
      continue;
    }
    const topIndices = Array.from(Array(x).keys(), v => v).reverse();
    const leftIndices = Array.from(Array(y).keys(), v => v).reverse();
    const rightIndices = Array.from(Array(width - y - 1).keys(), v => v + y + 1);
    const bottomIndices = Array.from(Array(height - x - 1).keys(), v => v + x + 1);

    let seenOnTop = 0;
    topIndices.some((i) => {
      if (trees[i][y] >= trees[x][y]) {
        seenOnTop += 1;
        return true;
      }

      seenOnTop += 1;
      return false;
    });

    let seeOnLeft = 0;
    leftIndices.some((i) => {
      if (trees[x][i] >= trees[x][y]) {
        seeOnLeft += 1;
        return true;
      }

      seeOnLeft += 1;
      return false;
    });

    let seenOnRight = 0;
    rightIndices.some((i) => {
      if (trees[x][i] >= trees[x][y]) {
        seenOnRight += 1;
        return true;
      }

      seenOnRight += 1;
      return false;
    });

    let seenOnBotton = 0;
    bottomIndices.some((i) => {
      if (trees[i][y] >= trees[x][y]) {
        seenOnBotton += 1;
        return true;
      }

      seenOnBotton += 1;
      return false;
    });

    const scenicScore = seenOnTop * seeOnLeft * seenOnRight * seenOnBotton;
    highestScenicScore = Math.max(highestScenicScore, scenicScore);
  }
}

console.log("🚀 ~ file: a.ts:73 ~ highestScenicScore", highestScenicScore)
