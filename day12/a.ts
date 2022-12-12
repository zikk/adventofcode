const input = await Deno.readTextFile('./input.test');

const heights = 'abcdefghijklmnopqrstuvwxyz'.split('');

class Node {
  constructor(public position: [number, number], public children: Node[] = []) {
    //
  }
}

const hill = input.split('\n').map((line) => line.split('').map((char) => {
  if (char === 'S') return 0;
  if (char === 'E') return 25;
  const index = heights.indexOf(char);
  return index;
})).map((row, y) => row.map((height, x) => new Node([x, y])))

console.log("🚀 ~ file: a.ts:18 ~ hill ~ hill", hill)
