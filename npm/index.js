#!/usr/bin/env node

const { spawnSync } = require("child_process");
const { cosmiconfig } = require("cosmiconfig");

const args = process.argv.slice(2);
const arch = process.arch;
const [os, extension] = ["win32", "cygwin"].includes(process.platform)
  ? ["windows", ".exe"]
  : [process.platform, ""];
const optionalDep = `deploytest-${os}-${arch}`;
const pkgSpecifier = `${optionalDep}/bin/deploytest${extension}`;

cosmiconfig("deploytest")
  .search()
  .then(({ config }) => (config ? JSON.stringify(config) : "{}"))
  .catch(() => "{}")
  .then((rcfileAsJson) => ({
    pathToBinary: require.resolve(pkgSpecifier),
    rcfileAsJson,
  }))
  .catch((err) => {
    panic(
      `expected optionalDependency "${optionalDep}" containing a Rust binary at "${pkgSpecifier}"`,
      err,
    );
  })
  .then(({ pathToBinary, rcfileAsJson }) => {
    process.exit(
      spawnSync(pathToBinary, args, {
        input: rcfileAsJson,
        stdio: "inherit",
      }).status ?? 0,
    );
  })
  .catch((err) => {
    panic("deploytest encountered an unknown error", err);
  });

function panic(message, err) {
  console.error(
    "\x1b[31m%s\n%s\x1b[0m",
    `! ${message}`,
    "  Please raise issue at https://github.com/JamieMason/deploytest/issues/new?template=bug_report.yaml",
    err,
  );
  process.exit(1);
}
