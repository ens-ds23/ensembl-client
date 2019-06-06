import React, { PureComponent } from 'react';

import errorService from 'src/services/error-service';

type FallbackComponentProps = {
  error: Error;
};

type Props = {
  fallbackComponent: React.ComponentType<FallbackComponentProps>;
  children: React.ReactNode | React.ReactNode[];
};

type State = {
  error: Error | null;
};

class ErrorBoundary extends PureComponent<Props, State> {
  public state = {
    error: null
  };

  private errorService = errorService;

  public static getDerivedStateFromError(error: Error) {
    return { error };
  }

  public componentDidCatch(error: Error) {
    this.errorService.report(error);
  }

  public render() {
    const FallbackComponent = this.props.fallbackComponent;
    const { error } = this.state;

    return error ? (
      <FallbackComponent error={error} />
    ) : (
      <>{this.props.children}</>
    );
  }
}

export default ErrorBoundary;
