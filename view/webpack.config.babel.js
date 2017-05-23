import path from 'path';
import webpack from 'webpack';
import HtmlWebpackPlugin from 'html-webpack-plugin';

export default {
  entry: {
    bundle: [
      'babel-polyfill',
      './lib/index.js'
    ]
  },
  output: {
    path: path.join(__dirname, '../mc-api/statics/'),
    filename: '[name].js'
  },
  resolve: {
    extensions: [".jsx", ".json", ".js"],
    alias: {
      '~': path.join(__dirname, '/lib/'),
      components: path.join(__dirname, '../common/components/')
    }
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
        options: {
          presets: [
            ['env', [
              {
                targets: {'browsers': ['last 1 versions']},
              }
            ]],
            'react'
          ],
          plugins: [
            'dynamic-import-webpack'
          ]
        }
      },
      {
        test: /\.pug$/,
        exclude: /node_modules/,
        loader: 'pug-loader'
      }
    ]
  },
  plugins: [
    new webpack.DefinePlugin({
      'process.env': {
        'NODE_ENV': JSON.stringify(process.env.NODE_ENV),
        'V_PORT': JSON.stringify(process.env.V_PORT),
        'MC_PORT': JSON.stringify(process.env.MC_PORT)
      }
    }),
    new HtmlWebpackPlugin({template: 'src/index.pug', filename: 'index.html'}),
    new HtmlWebpackPlugin({template: 'src/index.pug', filename: 'tasks.html'})
  ]
}
