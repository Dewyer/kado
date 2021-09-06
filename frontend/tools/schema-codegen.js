const cliLib = require("@codegena/oapi3ts-cli");
const path = require("path");

const cliApp = new cliLib.CliApplication();
const srcPath = path.resolve(__dirname, `../src/typings/schemas/api-schema.json`);
const message = `==> Api schema generated successfully...`;

cliApp.cliConfig.typingsDirectory = "";
cliApp.cliConfig.srcPath = srcPath;
cliApp.createTypings(cliApp.cliConfig);

console.log(message);
