const path = require("path");
const TerserPlugin = require("terser-webpack-plugin");

module.exports = {
  module: {
    rules: [
      {
        loader: "babel-loader",
        exclude: /node_modules/,
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
  optimization: {
    minimizer: [new TerserPlugin()]
  },
  resolve: {
    extensions: [".ts", ".js"]
  }
};
