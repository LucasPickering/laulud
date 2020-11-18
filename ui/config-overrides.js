const ESLintWebpackPlugin = require("eslint-webpack-plugin");

module.exports = function override(config) {
  // Remove the eslint webpack plugin, cause it sucks. who tf wants lints in the browser
  config.plugins = config.plugins.filter(
    (plugin) => !(plugin instanceof ESLintWebpackPlugin)
  );

  return config;
};
