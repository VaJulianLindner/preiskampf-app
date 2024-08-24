#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

// TODO only update if js/css was actually touched.
const timeStamp = new Date().getTime();
const cssBuildName = "./assets/dist/index.css";
const cssFileName =`./assets/dist/index.${timeStamp}.css`;
const jsBuildName = "./assets/dist/main.js";
const jsFileName = `./assets/dist/main.${timeStamp}.js`;

try {
    const files = fs.readdirSync(path.resolve("./assets/dist"));
    files.forEach(file => {
        if (!["index.css", "main.js"].includes(file)) {
            fs.rmSync(path.resolve(`./assets/dist/${file}`));
        }
    });
} catch (e) {
    console.error(e);
}

try {
    // will only rename if the files were newly built by webpack, else the clean name wont be found
    fs.renameSync(path.resolve(cssBuildName), path.resolve(cssFileName));
    fs.renameSync(path.resolve(jsBuildName), path.resolve(jsFileName));

    const headIncluePaths = [
        path.resolve("./templates/partials/head_include.html"),
        path.resolve("./templates_old/partials/head_include.hbs"),
    ];
    
    headIncluePaths.forEach(headIncluePath => {
        const headInclude = fs.readFileSync(headIncluePath, "utf8").toString().split("\n");
        headInclude.forEach((line, index) => {
            if (line.includes(jsBuildName.substring(1))) {
                headInclude[index] = `<script src="${jsFileName.substring(1)}"></script>`;
            } else if (line.includes(cssBuildName.substring(1))) {
                headInclude[index] = `<link rel="stylesheet" href="${cssFileName.substring(1)}">`;
            }
        });

        fs.writeFileSync(headIncluePath, headInclude.join("\n"));
    });
} catch (e) {
    console.warn(`${e.toString()}, the dist probably got renamed already`);
}
