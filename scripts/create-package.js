const { name, author, version, repository, description, keywords } = require("../package.json");
const fs = require("fs");
const path = require("path");

const json = {
  name,
  author,
  version,
  repository,
  description,
  keywords,
  module: "index.js",
  types: "index.d.ts",
  sideEffects: false,
};

fs.writeFileSync(path.join(process.cwd(), "pkg", "package.json"), JSON.stringify(json, null, 2));
