const input = await Deno.readTextFile('./input.prod');

const files: Record<string, Record<string, any>> = {
  '/': {},
};

let currentDir = "/";

input.split('\n').forEach((line) => {
  if (line.startsWith('$')) {
    const [_, command, param] = line.split(' ');

    if (command === 'cd') {
      switch (param) {
        case "/":
          currentDir = "/";
          break;

        case "..":
          currentDir = `/${currentDir.split('/').slice(0, 0)}`;
          break;

        default:
          currentDir = `${currentDir}${currentDir === '/' ? '' : '/'}${param}`;
          break;
      }

      if (!files[currentDir]) {
        files[currentDir] = {};
      }
    }
  } else {
    if (!line.startsWith('dir')) {
      const [fileSize, fileName] = line.split(' ');
        files[currentDir] = {
          ...files[currentDir],
          [fileName]: Number.parseInt(fileSize, 10),
        }
    }
  }
});

function calcDirSize(dir: string): number {
  const dirTotal = Object.values(files[dir]).reduce((total, size) => total + size, 0);
  const subDirs = Object.keys(files).filter((d) => d.startsWith(dir) && d.length > dir.length)
  const subDirTotal = subDirs.reduce((total, subDir) => {
    return total += calcDirSize(subDir);
  }, 0);

  return dirTotal + subDirTotal;
}

const result = Object.keys(files).reduce<Record<string, any>>((total, dir) => {
  if (dir === '/') return total;
  const dirTotal = calcDirSize(dir);

  return {
    ...total,
    [dir]: dirTotal,
  };
}, {})

console.log("🚀 ~ file: a.ts:64 ~ result ~ result", Object.keys(result).reduce((total, dir) => {
  const size: number = result[dir];
  return total += size <= 100_000 ? size : 0;
}, 0))
