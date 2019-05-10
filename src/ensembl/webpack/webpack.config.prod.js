const webpack = require('webpack');
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const TerserPlugin = require('terser-webpack-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const ManifestPlugin = require('webpack-manifest-plugin');
const CompressionPlugin = require('compression-webpack-plugin');
const BrotliPlugin = require('brotli-webpack-plugin');
const WorkboxPlugin = require('workbox-webpack-plugin');
const RobotstxtPlugin = require('robotstxt-webpack-plugin');

// copy from the environment the same variables that are declared in .env.example
// NOTE: if no environment variable with corresponding key is present, the value from .env.example will be used
const dotenv = require('dotenv').config({ path: path.resolve(__dirname, '../.env.example') });
const getEnvironmentVariables = () => Object.keys(dotenv.parsed).reduce((result, key) => ({
  [`process.env.${key}`]: JSON.stringify(process.env[key])
}));

// loaders specific to prod
const moduleRules = [
  // loader for images
  // image loader should compress the images
  // then file loader takes over to copy the images into the dist folder
  {
    test: /static\/img\/.*\.(gif|png|jpe?g)$/i,
    use: [
      {
        loader: 'file-loader',
        options: {
          emitFile: true,
          name: '[path][name].[hash].[ext]'
        }
      },
      'image-webpack-loader'
    ]
  },

  // loader for fonts that copies the fonts into the dist folder
  {
    test: /static\/fonts\/.*\.(woff2?|eot|ttf|otf|svg)$/i,
    loader: 'file-loader',
    options: {
      emitFile: true,
      name: '[path][name].[hash].[ext]'
    }
  }
];

// plugins specific to prod
const plugins = [
  // make environment variables available on the client-side
  new webpack.DefinePlugin({
    ...getEnvironmentVariables()
  }),

  // plugin to extract css from the webpack javascript build files
  new MiniCssExtractPlugin({
    filename: '[name].[contenthash].css',
    chunkFilename: '[id].[contenthash].css'
  }),

  // copy the browser assets into the production dist/ directory
  // this is only temporarily until a better solution is found
  new CopyWebpackPlugin([
    {
      from: path.join(__dirname, '../static/browser/browser.wasm'),
      to: path.join(__dirname, '../dist/static/browser/browser.wasm')
    }
  ]),

  // generate unique hashes for files based on the relative paths
  new webpack.HashedModuleIdsPlugin(),

  new CompressionPlugin({
    test: /.(js|css|html|wasm)$/,
    threshold: 5120,
    minRatio: 0.7
  }),

  // brotli compression for static files
  // only files above 5kB will be compressed
  new BrotliPlugin({
    test: /.(js|css|html|wasm)$/,
    threshold: 5120, // 5kB
    minRatio: 0.7
  }),

  // generate an asset manifest file that maps real file names with their cache-ready equivalents
  new ManifestPlugin(),

  // adds workbox library (from Google) support to enable service workers
  new WorkboxPlugin.GenerateSW({
    clientsClaim: true,
    skipWaiting: true
  }),

  new RobotstxtPlugin()
];

// prod specific configuration
const prodConfig = {
  optimization: {
    // use terser plugin instead of uglify js to support minimisation for modern React.js features
    // optimize css assets plugin to minimise css as it is not yet supported in webpack by default
    minimizer: [
      new TerserPlugin({
        cache: true,
        parallel: true,
        sourceMap: true
      }),
      new OptimizeCSSAssetsPlugin({})
    ],

    // create a separate webpack runtime chunk that will be used for all bundles
    runtimeChunk: true,

    // the common chunks configuration
    splitChunks: {
      cacheGroups: {
        // commonly shared code i.e. vendor code to be bundled as vendor.js
        commons: {
          test: /node_modules/,
          name: 'vendors',
          chunks: 'all'
        }
      }
    }
  },

};

// get the common config
const commonConfig = require('./webpack.common')(false, moduleRules, plugins);

// concatenate the common config with the prod config
module.exports = Object.assign({}, commonConfig, prodConfig);
