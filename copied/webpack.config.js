const path = require('path');

module.exports = {
  entry: './src/intro-to-3d.ts',
  mode: 'development',
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.ts'],
  },
  output: {
    filename: 'canvas_render.js',
    path: path.resolve(__dirname, 'dist'),
  },
};
