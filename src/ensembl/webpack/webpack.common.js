const path = require('path');
const postcssPresetEnv = require('postcss-preset-env');
const HtmlPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const ForkTsCheckerPlugin = require('fork-ts-checker-webpack-plugin');

// exports an anonymous function
// params are:
// 1. isDev: boolean - is it the development build that needs to be returned
// 2. moduleRules: array - all the module rules specific to the dev/prod build
// 3. plugins: array - all the plugins specific to the dev/prod build
// returns: webpack common configuration for the build
module.exports = (isDev, moduleRules, plugins) => ({
  // the source map type
  // for dev it will be a line by line source map
  // for prod it will be a generic source map that may not be as accurate as the former
  devtool: isDev ? 'cheap-module-eval-source-map' : 'source-map',

  // the starting point of webpack bundling
  entry: {
    index: path.join(__dirname, '../src/index.tsx')
  },

  // the mode is what notifies webpack how the build should be made
  mode: isDev ? 'development' : 'production',

  // the module loaders for webpack
  // these loaders help webpack to identify and process different file formats for bundling
  module: {
    rules: [
      // babel-loader is used for typescript as it has good tree shaking support compared to tsc
      {
        test: /.tsx?$/,
        loader: 'babel-loader',
        exclude: /node_modules/
      },

      // this will create source maps for the typescript code
      {
        test: /.js$/,
        exclude: /node_modules/,
        loader: 'source-map-loader',
        enforce: 'pre'
      },

      // the loaders for styling
      // there are two sets of them: for global and component styles
      // a scss file will first be loaded via sass loader and transpiled
      // afterwards it will be processed by postcss loader to make the css cross-browser compatible
      // add the processed css into the html document during runtime for dev
      // and extract the css for prod and minify it as external stylesheets
      {
        test: /.scss$/,
        include: /src\/ensembl\/src(?!\/styles)/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              sourceMap: true,
              modules: {
                localIdentName: '[local]__[name]__[hash:base64:5]'
              },
            }
          },
          {
            loader: 'postcss-loader',
            options: {
              ident: 'postcss',
              plugins: () => [postcssPresetEnv()]
            }
          },
          'sass-loader'
        ]
      },

      {
        test: /.scss$/,
        include: /src\/ensembl\/src\/styles/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              importLoaders: 2,
              sourceMap: true
            }
          },
          {
            loader: 'postcss-loader',
            options: {
              ident: 'postcss',
              plugins: () => [postcssPresetEnv()]
            }
          },
          'sass-loader'
        ]
      },

      // use file-loader on svg's (to be able to require them as a path to the image),
      // but also use @svgr/webpack to be able to require svg's directly as React components
      {
        test: /\.svg$/,
        use: ['@svgr/webpack', 'file-loader'],
      },

      // loaders specific to dev/prod
      ...moduleRules
    ]
  },

  // prevent webpack from searching fs (node API) to load the web assembly files
  node: {
    fs: 'empty'
  },

  // this is the config to define how the output files needs to be
  output: {
    // dev will take the default file names as no physical files are emitted
    // prod will have emitted files and will include the content hash, which will change every time the contents of the js file changes.
    filename: isDev ? undefined : '[name].[contenthash].js',

    path: path.resolve(__dirname, '../dist/static'),

    // stop webpack from adding additional comments/info to generated bundles as it is a performance hit (slows down build times)
    pathinfo: false,

    // prepend the public path as the root path to all the files that are inserted into the index file
    publicPath: isDev ? '/' : '/static/'
  },

  // the plugins that extends the webpack configuration
  plugins: [
    // checks typescript types
    new ForkTsCheckerPlugin(),

    // generates the index file using the provided html template
    new HtmlPlugin({
      // in prod, path for saving static assets is dist/static/, and index.html has to be saved top-level in the dist folder
      filename: isDev ? 'index.html' : '../index.html',
      template: path.join(__dirname, '../static/html/template.html'),
      publicPath: '/'
    }),

    // plugins specific to dev/prod
    ...plugins
  ],

  // configuration that allows us to not to use file extensions and shorten import paths (using aliases)
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.scss'],
    alias: {
      config: path.resolve(__dirname, '../config.ts'),
      src: path.join(__dirname, '../src'),
      tests: path.join(__dirname, '../tests'),
      static: path.join(__dirname, '../static')
    }
  }
});
