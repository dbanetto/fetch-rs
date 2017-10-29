const webpack = require('webpack');
const path = require('path');
const ExtractTextPlugin = require("extract-text-webpack-plugin");

let production = process.env.ROCKET_ENV === "production";

const extractSass = new ExtractTextPlugin({
  filename: "[name].css"
});


module.exports = {
  cache: true,
  entry: {
    main: './bundle/main.jsx',
    style: './bundle/style.scss',
    vendor: [
      'preact',
      'preact-router',
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
        test: /\.jsx?$/,
        exclude: /node_modules/,
        loader: 'babel-loader?presets[]=es2015'
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

    new webpack.optimize.CommonsChunkPlugin({
      name: "vendor", filename: "vendor.js", minChunks: Infinity}),

    extractSass,

  ].concat(production ?
    [
      // production plugins
      new webpack.optimize.UglifyJsPlugin({
        compress: { warnings: false, drop_console: false, }
      })
    ] : [
      // development plugins
    ]

  ),
  resolve: {
    extensions: ['.js', '.jsx', '.css', '.scss']
  },
};
