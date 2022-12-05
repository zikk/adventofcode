const input = await Deno.readTextFile('./input.prod');

const alphabetUpperAndLower = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

const groups = [];

for (let i = 0; i < input.split('\n').length; i += 3) {
  groups.push([input.split('\n')[i], input.split('\n')[i + 1], input.split('\n')[i + 2]]);
}

const result = groups.reduce((total, group) => {
  const commonItem = group[0].split('').filter((letter) => group[1].split('').includes(letter) && group[2].split('').includes(letter))[0];
    return total + alphabetUpperAndLower.indexOf(commonItem) + 1;
}, 0);

console.log(result);
