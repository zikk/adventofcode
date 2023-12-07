import minimist from "minimist";
import c from "ansi-colors";

const argv = minimist(process.argv.slice(2));

const day = argv.day || argv.d;
const part = argv.part || argv.p;

if (!day || !part) {
  console.log("Please provide day and part as arguments");
  process.exit(1);
}

const filePath = `./src/ts/day${day}_${part}.ts`;
const file = Bun.file(filePath);

file.exists().then(async (exists) => {
  if (exists) {
    console.log(c.green(`Running day ${day} part ${part}`));
    console.log(c.cyan("<====================================>"));

    try {
      await import(filePath);
    } catch (e) {
      console.error(c.red(`Error running day ${day} part ${part}`));
    }

    console.log(c.cyan("<====================================>"));
  } else {
    console.error(c.red("⚠️  File not found ⚠️"));
    process.exit(1);
  }
});
