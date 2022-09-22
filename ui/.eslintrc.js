module.exports = {
  extends: ["@lucaspickering/eslint-config/react"],
  rules: {
    "react/no-unknown-property": [
      "error",
      {
        ignore: [
          "css", // Added by emotion
        ],
      },
    ],
    "no-restricted-syntax": [
      "error",
      {
        selector:
          "ImportDeclaration[source.value=react-relay] > ImportSpecifier[imported.name=useMutation]",
        message:
          "Use the local useMutation wrapper instead of the one from react-relay",
      },
    ],
  },
};
