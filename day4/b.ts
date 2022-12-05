const input = await Deno.readTextFile('./input.prod');

const result = input.split('\n').reduce((total, line) => {
  const firstSect = line.split(',')[0];
  const secondSection = line.split(',')[1];

  const firstSecNumbers = firstSect.split('-').map((n) => parseInt(n, 10));
  const secondSecNumbers = secondSection.split('-').map((n) => parseInt(n, 10));

  if (
    firstSecNumbers[0] >= secondSecNumbers[0] && firstSecNumbers[0] <= secondSecNumbers[1]
    || firstSecNumbers[1] <= secondSecNumbers[1] && firstSecNumbers[1] >= secondSecNumbers[0]
    || secondSecNumbers[1] <= firstSecNumbers[1] && secondSecNumbers[1] >= firstSecNumbers[0]
    || secondSecNumbers[1] <= firstSecNumbers[1] && secondSecNumbers[1] >= firstSecNumbers[0]
  ) {
    return total + 1;
  } else {
    return total;
  }
}, 0);

console.log(result);