import { BreakpointWidth } from './globalConfig';

export const getBreakpoint = (width: number): BreakpointWidth => {
  if (width > BreakpointWidth.LARGE) {
    return BreakpointWidth.LARGE;
  } else if (width > BreakpointWidth.MEDIUM) {
    return BreakpointWidth.MEDIUM;
  } else {
    return BreakpointWidth.SMALL;
  }
};

export const getQueryParamsMap = (queryStr: string) => {
  const params = queryStr.replace('?', '').split('&'); // split all query param pairs

  let paramMap: { [key: string]: string } = {};

  params.forEach((param: string) => {
    const [key, value] = param.split('='); // split param pair into key value

    paramMap[key] = value; // add the param pair into the param map
  });

  return paramMap;
};
