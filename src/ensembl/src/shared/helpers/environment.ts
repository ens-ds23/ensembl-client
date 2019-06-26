import config from 'config';

export enum Environment {
  DEVELOPMENT = 'development',
  INTERNAL = 'internal',
  PRODUCTION = 'production'
}

export const isEnvironment = (environment: Environment[]): boolean => {
  return environment.includes(config.environment as Environment);
};
