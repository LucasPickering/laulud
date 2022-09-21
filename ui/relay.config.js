module.exports = {
  src: "./src",
  schema: "./schema.graphql",
  language: "typescript",
  exclude: ["**/node_modules/**", "**/__generated__/**"],
  customScalars: {
    SpotifyUri: "string",
    Tag: "string",
  },
};
