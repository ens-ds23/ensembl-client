import { isEnvironment, Environment } from './environment';
import config from 'config';

describe('isEnvironment', () => {
  // Set the environment to development
  config.environment = 'development';

  it('returns true if the envrionment matches', () => {
    const isAvailableOnEnvironment = isEnvironment([Environment.DEVELOPMENT]);

    expect(isAvailableOnEnvironment).toBe(true);
  });

  it('returns false if the envrionment does not match', () => {
    const isAvailableOnEnvironment = isEnvironment([Environment.PRODUCTION]);

    expect(isAvailableOnEnvironment).toBe(false);
  });

  it('suppoerts multiple environments as input and returns true if at least one matches', () => {
    const isAvailableOnEnvironment = isEnvironment([
      Environment.DEVELOPMENT,
      Environment.INTERNAL
    ]);

    expect(isAvailableOnEnvironment).toBe(true);
  });
});
