module.exports = {
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
  moduleNameMapper: {
    '\\.svg': '<rootDir>/tests/svgrMock.js',
    '^config$': '<rootDir>/config.ts',
    '(tests/.*)$$': '<rootDir>/$1',
    '(src/.*)$': '<rootDir>/$1',
    '(static/.*)$': '<rootDir>/$1',
    '(static/browser/.*)$': '<rootDir>/$1.stub.js',
    '\\.(css|scss)$': 'identity-obj-proxy'
  },
  roots: ['<rootDir>/src'],
  setupFilesAfterEnv: ['<rootDir>/setup-enzyme.ts'],
  testEnvironment: 'jsdom',
  transform: {
    '.+\\.tsx?$': 'babel-jest',
    '.+\\.(css|scss|png|jpg|svg|gif|eot|ttf|otf|woff|woff2)$':
      'jest-transform-stub'
  }
};
