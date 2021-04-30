module.exports = {
  src: "./src",
  schema: "./schema.graphql",
  language: "typescript",
  watchman: false, // Don't use watchman while not watching files
  exclude: ["**/node_modules/**", "**/__generated__/**"],
};
