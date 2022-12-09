const input = await Deno.readTextFile('./input.test');

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

const result = Object.keys(files).reduce<Record<string, any>>((total, dir) => {
  if (dir.split('/').length === 2) {
    const dirTotal = Object.values(files[dir]).reduce((total, size) => total + size, 0);
    const subDirs = Object.keys(files)
    .filter((d) => d.startsWith(dir) && d.length > dir.length)
    const subDirTotal = subDirs.reduce((total, subDir) => {
      return total += Object.values(files[subDir]).reduce((total, size) => total + size, 0);
    }, 0);

    return {
      ...total,
      [dir]: dirTotal + subDirTotal,
    };
  } else {
    return total;
  }
}, {})

console.log("🚀 ~ file: a.ts:51 ~ result ~ result", Object.keys(result).reduce((total, dir) => {
  const size = result[dir];
  console.log("🚀 ~ file: a.ts:64 ~ console.log ~ size", size)

  if (size <= 100_000) {
    return total += size;
  }

  return total;
}, 0))
