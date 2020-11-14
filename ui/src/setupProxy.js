const { createProxyMiddleware } = require("http-proxy-middleware");

const target = process.env.LAULUD_API_HOST;
if (!target) {
  throw new Error("No proxy target defined. Set LAULUD_API_HOST.");
}

module.exports = function (app) {
  app.use(createProxyMiddleware("/api", { target }));
};
