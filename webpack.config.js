const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { GenerateSW } = require('workbox-webpack-plugin');

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  plugins: [
    new GenerateSW({
      // include: [/\.html$/, /\.js$/, /\.css$/, /\.png$/, /\.wasm $/],
      // globPatterns: ['**/*.{js,css,html}'],
      globDirectory: './',
      runtimeCaching: [
        {
          urlPattern: /^http[s|]?.*/,
          handler: 'staleWhileRevalidate'
        }
      ],
      clientsClaim: true,
      skipWaiting: true
    }),
    new CopyWebpackPlugin([{ from: 'static', to: 'static' }]),
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.')
    }),
    new webpack.ProvidePlugin({
      TextDecoder: ['text-encoding', 'TextDecoder'],
      TextEncoder: ['text-encoding', 'TextEncoder']
    })
  ],
  mode: 'development'
};
