const input = await Deno.readTextFile('./input.prod');

type Monkey = {
  items: number[];
  operation: string;
  testNumber: number;
  targets: number[];
  inspections: number;
}

const monkeys = input.split('\n').reduce<Monkey[]>((acc, line) => {
  if (line.startsWith('M')) {
    acc.push({ inspections: 0 } as Monkey);
  } else if (line.trim().startsWith('Starting')) {
    acc[acc.length - 1]['items'] = line.split(':')[1].split(',').map((item) => Number.parseInt(item.trim(), 10));
  } else if (line.trim().startsWith('Operation')) {
    acc[acc.length - 1]['operation'] = line.split(':')[1].trim();
  } else if (line.trim().startsWith('Test')) {
    acc[acc.length - 1]['testNumber'] = Number.parseInt(line.split(':')[1].split(' ')[3].trim(), 10);
  } else if (line.trim().startsWith('If')) {
    const monkeyIndex = Number.parseInt(line.split(':')[1].split(' ')[4].trim(), 10);
    acc[acc.length - 1]['targets'] = acc[acc.length - 1]['targets'] ? [...acc[acc.length - 1]['targets'], monkeyIndex] : [monkeyIndex];
  }

  return acc;
}, []);

for (let index = 0; index < 20; index += 1) {
  monkeys.forEach((monkey, mIndex) => {
    const removedItems: number[] = [];
    monkey.items.forEach((item) => {
      const old = item;
      let newNum = old;
      const operation = monkey.operation.replace('new', 'newNum');
      eval(operation);
      const newWorryLevel = Math.floor(newNum / 3);
      if (newWorryLevel % monkey.testNumber === 0) {
        monkeys[monkey.targets[0]].items.push(newWorryLevel);
      } else {
        monkeys[monkey.targets[1]].items.push(newWorryLevel);
      }

      removedItems.push(item);
      monkeys[mIndex].inspections += 1;
    });

    monkeys[mIndex].items = monkey.items.filter((item) => !removedItems.includes(item));
  });
}

const topInspections = Object.values(monkeys).map((monkey) => monkey.inspections).sort((a, b) => b - a).slice(0, 2);
console.log(topInspections.reduce((total, num) => total * num, 1));
