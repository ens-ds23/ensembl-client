module.exports = {
  presets: [
    '@babel/react',
    '@babel/typescript',
    [
      '@babel/env',
      {
        useBuiltIns: 'usage',
        corejs: 3,
        modules: false
      }
    ]
  ],
  plugins: [
    '@babel/plugin-syntax-dynamic-import',
    '@babel/plugin-proposal-object-rest-spread',
    '@babel/plugin-proposal-class-properties'
  ],
  env: {
    test: {
      presets: [
        [
          '@babel/env',
          {
            useBuiltIns: 'usage',
            corejs: 3
          }
        ]
      ]
    }
  }
};
