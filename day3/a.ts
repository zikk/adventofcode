const input = await Deno.readTextFile('./input.prod');

const alphabetUpperAndLower = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

const result = input.split('\n').reduce((total, sack) => {
  const sackArr = sack.split('');
  const firstSide = sackArr.splice(0, sackArr.length / 2);
  const secondSide = [...sackArr];
  const commonItem = firstSide.filter((letter) => secondSide.includes(letter))[0];
  return total + alphabetUpperAndLower.indexOf(commonItem) + 1;
}, 0);

console.log(result);