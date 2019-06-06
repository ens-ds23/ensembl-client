// TODO: add a real error monitoring service
// Sentry (can be self-hosted)? Rollbar (allows unlimited users in its free plan)?

interface ErrorServiceInterface {
  report: (error: Error) => void;
}

class ErrorService implements ErrorServiceInterface {
  public report(error: Error) {
    // TODO: actually report the errors somehow
    console.log('error', error);
  }
}

export default new ErrorService();
