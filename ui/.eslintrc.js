module.exports = {
  env: {
    node: true, // for "process"
    browser: true,
    es6: true,
  },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    project: "./tsconfig.json",
  },
  plugins: [
    "@typescript-eslint",
    "react",
    "react-hooks",
    "prettier",
    "jsx-a11y",
  ],
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react/recommended",
    "plugin:prettier/recommended",
    "plugin:jsx-a11y/recommended",
  ],
  globals: {
    Atomics: "readonly",
    SharedArrayBuffer: "readonly",
  },
  settings: {
    react: {
      pragma: "React",
      version: "detect",
    },
  },
  rules: {
    "no-console": "warn",
    "no-unused-vars": "off",
    "react/prop-types": "off",
    "react-hooks/rules-of-hooks": "error",
    "react-hooks/exhaustive-deps": "error",
    "react/display-name": "off",
    "jsx-a11y/no-autofocus": "off",
    "@typescript-eslint/no-unused-vars": "error",
    "@typescript-eslint/no-explicit-any": ["error", { fixToUnknown: true }],
    "@typescript-eslint/explicit-function-return-type": [
      "error",
      { allowExpressions: true, allowTypedFunctionExpressions: true },
    ],
    "@typescript-eslint/no-object-literal-type-assertion": "off",
    "@typescript-eslint/no-inferrable-types": [
      "error",
      { ignoreParameters: true },
    ],
    "@typescript-eslint/camelcase": "off",
  },
  overrides: [
    {
      // Special config files
      files: ["*.js"],
      parserOptions: {
        ecmaVersion: 3,
      },
      rules: {
        "@typescript-eslint/no-var-requires": "off",
      },
    },
    {
      files: ["*.test.ts"],
      plugins: ["jest"],
      env: {
        jest: true,
      },
    },
  ],
};
