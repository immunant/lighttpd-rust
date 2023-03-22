#!/usr/bin/env node

import * as fsp from "fs/promises";
import * as pathLib from "path";
import * as assert from "assert/strict";

function amalgamatedPath(extension) {
    return pathLib.resolve(`lighttpd.amalgamated.${extension}`);
}

const commands = JSON.parse((await fsp.readFile("compile_commands.json")).toString())
    .filter(e => pathLib.relative(".", e.directory) === "build")
    .filter(e => ![
        // These are for other binaries and so have their own `main`s, which conflict.
        "lemon.c",
        // "lighttpd-angel.c",
    ].includes(pathLib.relative("./src", e.file)))
    ;

const includes = [
    // For some reason, this needs to be first in order to define `XXH32`.
    "src/algo_xxhash.c",
    ...commands.map(e => pathLib.relative(".", e.file))
]
    .map(path => `#include "${path}"`)
    .join("\n")
    ;

const flags = [...new Set(
    commands.flatMap(e => (e.arguments ?? e.command.split(/ +/g))
        .slice(
            ["cc"].length, // Slice off the compiler invocation (not a flag).
            -["-o", "obj", "-c", "src"].length, // Slice off the file-specific args that aren't flags.
        )
    )
)];

const directories = [...new Set(commands.map(e => e.directory))];
assert.equal(directories.length, 1);
const directory = directories[0];

const amalgamated = {
    exe: {
        directory,
        arguments: ["cc", ...flags, "-o", amalgamatedPath("exe"), amalgamatedPath("c"), "-lpcre2-8", "-ldl", "-Wl,-export-dynamic"],
        file: amalgamatedPath("c"),
    },
    ii: {
        // A post-preprocessing file, just to check exactly what's included or not.
        directory,
        arguments: ["cc", ...flags, "-o", amalgamatedPath("ii"), "-E", amalgamatedPath("c")],
        file: amalgamatedPath("c"),
    },
};

const amalgamatedCommands = [amalgamated.exe];

await fsp.writeFile(amalgamatedPath("c"), includes);
await fsp.writeFile("amalgamated.compile_commands.json", JSON.stringify(amalgamatedCommands, null, 4));

function toShellCommand(cmd) {
    const command = cmd.arguments && cmd.arguments.join(" ") || cmd.command;
    return `(cd "${cmd.directory}" && ${command})`;
}

console.log(toShellCommand(amalgamated.ii));
console.log(toShellCommand(amalgamated.exe));
