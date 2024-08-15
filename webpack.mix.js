const path = require("path");

module.exports = {
    mode: process.argv.indexOf("--watch") > -1 ? "development" : "production",
    watch: process.argv.indexOf("--watch") > -1,
    entry: "./resources/js/main.js",
    output: {
        filename: "main.js",
        path: path.resolve(__dirname, "assets/dist"),
    },
};