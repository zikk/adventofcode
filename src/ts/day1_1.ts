const input = await Deno.readTextFile('./day1.prod');

const calories: number[] = input
  .split('\n\n').reduce<number[]>((acc, line) => {
    acc
      .push(line.split('\n')
      .reduce((total: number, value: string) => { total += Number.parseInt(value, 10); return total }, 0) as number);
    return acc;
  }, [] as number[])
  .sort((a, b) => {
    if (a < b) {
      return 1;
    }

    if (a > b) {
      return -1;
    }

    return 0;
  });

console.log(calories[0]);
