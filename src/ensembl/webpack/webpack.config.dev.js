const dotenv = require('dotenv').config();

const webpack = require('webpack');
const path = require('path');
const StylelintWebpackPlugin = require('stylelint-webpack-plugin');

// laoders specific to dev
const moduleRules = [
  // this is the loader that will make webpack load file formats that are not supported by other loaders
  {
    test: /\.(woff|woff2|eot|ttf|otf|gif|png|jpe?g)$/,
    loader: 'file-loader',
    options: {
      // the file path and name that webpack will use to store these files
      name: `[path][name].[ext]`
    }
  }
];

// plugins specific to dev
const plugins = [
  // this plugin is required to enable hot module reloading
  // for the webpack dev server
  new webpack.HotModuleReplacementPlugin(),

  // lint the SASS files within the Ensembl codebase only
  new StylelintWebpackPlugin({
    context: path.join(__dirname, '../src'),
    files: '**/*.scss'
  }),

  // make environment variables available on the client-side
  new webpack.DefinePlugin({
    'process.env': JSON.stringify(dotenv.parsed)
  })
];

// dev specific configuration
const devConfig = {
  // speed up build times for dev
  optimization: {
    removeAvailableModules: false,
    removeEmptyChunks: false,
    splitChunks: false
  },

  // configuration to run webpack server for development
  devServer: {
    before(app) {
      // use proper mime-type for wasm files
      app.get('*.wasm', function(req, res, next) {
        var options = {
          root: path.join(__dirname, '..'),
          dotfiles: 'deny',
          headers: {
            'Content-Type': 'application/wasm'
          }
        };
        res.sendFile(req.url, options, function(err) {
          if (err) {
            next(err);
          }
        });
      });
    },

    // disable host check only when testing on IE11 remotely
    disableHostCheck: false,

    // fallback for the history API used by the react router when page is reloaded
    // this should prevent 404 errors that usually occur in SPA on reloads
    historyApiFallback: true,

    // make the server accessible from other machines
    host: '0.0.0.0',

    // enable hot module reloading
    hot: true,

    // configuration to customise what is displayed in the console by webpack
    stats: {
      assets: false,
      chunks: false,
      colors: true,
      modules: false
    }
  },

  // disable webpack from watching node modules
  // this would reduce memory consumption and also should improve build times
  watchOptions: {
    ignored: /node_modules/
  }
};

// get the common config
const commonConfig = require('./webpack.common')(true, moduleRules, plugins);

// concatenate the common config with the dev config
module.exports = Object.assign({}, commonConfig, devConfig);
