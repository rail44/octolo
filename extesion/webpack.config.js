const path = require("path");
const TerserPlugin = require("terser-webpack-plugin");

module.exports = {
  module: {
    rules: [
      {
        loader: "babel-loader",
        options: {
          presets: [
            [
              "@babel/preset-env",
              {
                modules: false
              }
            ]
          ]
        },
        test: /\.js$/
      },
      {
        test: /\.ts$/,
        loader: "ts-loader"
      }
    ]
  },
  entry: {
    background: "./src/background.ts",
    content: "./src/content.ts"
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "pkg/dist")
  },
  mode: "development",
  optimization: {
    minimizer: [new TerserPlugin()]
  }
};
