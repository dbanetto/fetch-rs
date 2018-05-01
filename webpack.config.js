const webpack = require('webpack');
const path = require('path');
const ExtractTextPlugin = require("extract-text-webpack-plugin");
const UglifyJsPlugin = require('uglifyjs-webpack-plugin')

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
      'preact-redux',
      'preact-router',
      'redux',
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
      },
      {
        test: /.(html|png|woff(2)?|eot|ttf|svg)(\?[a-z0-9=\.]+)?$/,
        loader: 'file-loader',
        options: {
          name: '[name].[ext]',
          publicPath: '/public',
        }
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
      new UglifyJsPlugin({
        test: /.js$/,
        sourceMap: true,
        uglifyOptions: {
          compress: true,
          warnings: false
        }
      })
    ] : [
      // development plugins
    ]

  ),
  resolve: {
    extensions: ['.js','.ts', '.tsx', '.css', '.scss']
  },
  watchOptions: {
    ignored: [
      /node_modules/,
      "src/backend/*",
      /target/,
      /.git/
    ],
  }
};
