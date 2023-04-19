const input = await Deno.readTextFile('./input.prod');

const result = input.split('').reduce<string[]>((stack, char) => {
  if (stack.length === 14) {
    return stack;
  }

  if (stack.includes(char)) {
    const charIndex = stack.findIndex((c) => c === char);
    stack = [...stack.slice(charIndex + 1), char];
    return stack;
  }

  stack.push(char);
  return stack;
}, [])

console.log(input.indexOf(result.join('')) + result.join('').length);
