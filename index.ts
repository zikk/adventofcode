import path from "path";
import minimist from "minimist";
import c from "ansi-colors";
import { $ } from 'execa';

const argv = minimist(process.argv.slice(2));

const ALLOWED_LANGS = [
  "ts",
  "go",
  "rust",
];

const ALLOWED_ENVS = ["test", "prod"];

const FILE_EXTENSIONS: Record<string, string> = {
  "ts": "ts",
  "go": "go",
  "rust": "rs",
}

const lang = argv.lang || argv.l || "ts";
const day = argv.day || argv.d;
const part = argv.part || argv.p;
const env = argv.environment || argv.env || argv.e || "prod";

if (!ALLOWED_LANGS.includes(lang)) {
  console.error(c.red(`Language ${lang} not supported`));
  process.exit(1);
}

if (!ALLOWED_ENVS.includes(env)) {
  console.error(c.red(`Environment ${env} not supported`));
  process.exit(1);
}

if (!day || !part) {
  console.log("Please provide day and part as arguments");
  process.exit(1);
}

const filePath = path.join(
  import.meta.dir,
  `./src/${lang === "rust" ? "bin" : lang}/${lang === 'go' ? `day${day}_${part}/` : ''}day${day}_${part}.${FILE_EXTENSIONS[lang]}`
);

const file = Bun.file(filePath);

async function runFile() {
  switch(lang) {
    case 'ts':
      await import(filePath);
      break;
    case 'go': {
        const { stdout, stderr } = await $`go run ${filePath} -e ${env}`;
        console.error(c.red(stderr));
        console.log(stdout);
        break;
      }
    case 'rust':
      const { stdout, stderr } = await $`cargo run --bin day${day}_${part} -- --env ${env}`;
      console.error(c.red(stderr));
      console.log(stdout);
      break;
  }
}

file.exists().then(async (exists) => {
  if (exists) {
    console.log(c.green(`Running day ${day} part ${part}`));
    console.log(c.cyan("<====================================>"));

    try {
      await runFile()
    } catch (e) {
      console.error(c.red(`Error running day ${day} part ${part}`));
      console.error(e);
    }

    console.log(c.cyan("<====================================>"));
  } else {
    console.error(c.red("⚠️  File not found ⚠️"));
    process.exit(1);
  }
});
