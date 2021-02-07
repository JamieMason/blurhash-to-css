module.exports = {
  clearMocks: true,
  moduleFileExtensions: ['node', 'js', 'ts'],
  testEnvironment: 'node',
  testMatch: ['<rootDir>/src/*.spec.(ts|js)'],
  transform: { '^.+\\.ts$': 'ts-jest' },
};
