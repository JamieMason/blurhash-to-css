module.exports = {
  clearMocks: true,
  moduleFileExtensions: ['ts', 'js'],
  testEnvironment: 'node',
  testMatch: ['<rootDir>/*.spec.(ts|js)'],
  transform: { '^.+\\.ts$': 'ts-jest' },
};
