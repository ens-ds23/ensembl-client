module.exports = {
  parser: '@typescript-eslint/parser',
  extends: [
    'plugin:@typescript-eslint/recommended',
    'plugin:react/recommended',
    'prettier/@typescript-eslint', // Uses eslint-config-prettier to disable rules that conflict with prettier
    'plugin:prettier/recommended' // Displays prettier errors as ESLint errors. This has to be the last in this array (not yet sure why).
  ],
  plugins: ['react-hooks'],
  parserOptions: {
    ecmaVersion: 2019,
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true
    }
  },
  // rules options: 0 = off, 1 = warn, 2 = error
  rules: {
    'no-console': [1, { allow: ['error', 'info', 'warn'] }],
    '@typescript-eslint/camelcase': 0,
    '@typescript-eslint/interface-name-prefix': 0,
    '@typescript-eslint/prefer-interface': 0,
    '@typescript-eslint/explicit-function-return-type': 0,
    '@typescript-eslint/no-explicit-any': 0,
    '@typescript-eslint/no-use-before-define': 0,
    'react/display-name': 0,
    'react/prop-types': 0,
    'react/no-unescaped-entities': 0,
    'react-hooks/rules-of-hooks': 2,
    'prettier/prettier': 0
  },
  settings: {
    react: {
      version: 'detect' // Makes eslint-plugin-react to detect the React version
    }
  }
};
