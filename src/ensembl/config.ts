export default {
  // version number
  version: '0.2.0',

  // environment
  isDevelopment: process.env.NODE_ENV === 'development',
  isProduction: process.env.NODE_ENV === 'production',
  apiHost: process.env.API_HOST,

  // keys for services
  googleAnalyticsKey: process.env.GOOGLE_ANALYTICS_KEY,

  // Genesearch endpoint
  genesearchAPIEndpoint: process.env.GENESEARCH_API_ENDPOINT
};
