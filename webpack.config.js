const webpack = require('webpack');
const path = require('path');

module.exports = {
  cache: true,
  entry: {
    main: './bundle/main.jsx',
    vendor: [
      'react',
      'react-dom',
      'react-router'
    ]
  },
  output: {
    path: path.resolve(__dirname, 'public', 'bundle'),
    filename: '[name].js',
    chunkFilename: '[chunkhash].js'
  },
  module: {
    rules: [
    {
      test: /\.js$/,
      exclude: /node_modules/,
      loader: 'babel-loader?presets[]=es2015'
    },
    {
      test: /\.jsx$/,
      exclude: /node_modules/,
      loader: 'babel-loader?presets[]=es2015&presets[]=react'
    }
    ]
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin({name: "vendor", filename: "vendor.js"})
  ],
  resolve: {
    extensions: ['.js', '.jsx']
  },
};
