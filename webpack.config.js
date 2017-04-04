const webpack = require('webpack');
const path = require('path');
const ExtractTextPlugin = require("extract-text-webpack-plugin");

const UglifyJsPlugin = process.env.ROCKET_ENV === "production" ? new webpack.optimize.UglifyJsPlugin() : null;
const extractSass = new ExtractTextPlugin({
    filename: "[name].css"
});

module.exports = {
  cache: true,
  entry: {
    main: './bundle/main.jsx',
    style: './bundle/style.scss',
    vendor: [
      'react',
      'react-dom',
      'react-router',
      'react-jsonschema-form'
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
    },
    {
      test: /\.scss$/,
      use: extractSass.extract({
        use: [{
          loader: "css-loader"
        }, {
          loader: "sass-loader"
        }],
          // use style-loader in development
          fallback: "style-loader"
      })
    }
    ]
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin({name: "vendor", filename: "vendor.js", minChunks: Infinity}),
    extractSass
  ],
  resolve: {
    extensions: ['.js', '.jsx', '.css', '.scss']
  },
};
