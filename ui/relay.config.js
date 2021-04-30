module.exports = {
  src: "./src",
  schema: "./schema.graphql",
  language: "typescript",
  watchman: false, // watchman blows, we use nodemon instead
  exclude: ["**/node_modules/**", "**/__generated__/**"],
  customScalars: {
    SpotifyUri: "string",
    Tag: "string",
  },
};
