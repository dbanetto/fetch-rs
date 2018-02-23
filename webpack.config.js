const webpack = require('webpack');
const path = require('path');
const ExtractTextPlugin = require("extract-text-webpack-plugin");

let production = process.env.ENV === "production";

const extractSass = new ExtractTextPlugin({
  filename: "[name].css"
});


module.exports = {
  cache: true,
  devtool: production ? '' : 'source-map',
  entry: {
    main: './src/frontend/main.tsx',
    style: './src/frontend/style.scss',
    vendor: [
      'preact',
      'preact-router',
    ]
  },
  output: {
    path: path.resolve(__dirname, 'public'),
    filename: '[name].js',
    chunkFilename: '[chunkhash].js'
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        loader: 'babel-loader'
      },
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        loader: "ts-loader"
      },
      {
        test: /\.(sass|scss)$/,
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
        test: /.jsx?$/,
        sourceMap: true,
        uglifyOptions: {
          compress: true,
          warnings: false
        }
      })
    ] : [
      // development plugins
      new webpack.SourceMapDevToolPlugin({
        test: /jsx?$/,
        filename: '[name].js.map',
        exclude: ['vendor.js']
      })
    ]

  ),
  resolve: {
    extensions: ['.js', '.jsx','.ts', '.tsx', '.css', '.scss']
  },
};
