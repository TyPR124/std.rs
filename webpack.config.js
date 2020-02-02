const path = require('path');

const context = __dirname;
const mode = process.env.NODE_ENV || 'production';

module.exports = {
  mode,
  context,
  entry: './src/index.ts',
  target: 'webworker',
  output: {
    filename: `worker.${mode}.js`,
    path: path.join(context, 'dist'),
  },
  devtool: 'source-map',
  resolve: {
    extensions: ['.ts', '.tsx', '.js'],
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: 'ts-loader',
      },
    ],
  },
};
