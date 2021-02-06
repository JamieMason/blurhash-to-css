module.exports = {
  clearMocks: true,
  moduleFileExtensions: ['ts', 'js'],
  testEnvironment: 'node',
  testMatch: ['<rootDir>/src/*.spec.(ts|js)'],
  transform: { '^.+\\.ts$': 'ts-jest' },
};
