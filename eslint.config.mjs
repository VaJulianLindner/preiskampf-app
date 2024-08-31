import globals from "globals";
import pluginJs from "@eslint/js";

export default [
    { 
        languageOptions: { 
            globals: {
                ...globals.browser,
                ...globals.commonjs,
                ...globals.node,
                H: "readable",
                Router: "readable",
            },
        } 
    },
    {
        rules: {
            ...pluginJs.configs.recommended.rules,
            "no-unused-vars": "off",
            indent: [
                "error",
                4,
                { SwitchCase: 1 },
            ]
        }
    }
]
