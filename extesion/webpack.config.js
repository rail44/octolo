const path = require("path");
const TerserPlugin = require("terser-webpack-plugin");

module.exports = {
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: "ts-loader"
      },
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
      }
    ]
  },
  entry: {
    background: "./src/background.ts"
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, 'pkg/dist')
  },
  mode: "development",
  optimization: {
    minimizer: [new TerserPlugin()]
  }
};
