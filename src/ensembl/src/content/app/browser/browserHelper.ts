import { ChrLocation } from './browserState';

export function getChrLocationFromStr(chrLocationStr: string): ChrLocation {
  const [chrCode, chrRegion] = chrLocationStr.split('=')[1].split(':');
  const [startBp, endBp] = chrRegion.split('-');

  return [chrCode, +startBp, +endBp];
}

export function getChrLocationStr(chrLocation: ChrLocation): string {
  const [chrCode, startBp, endBp] = chrLocation;

  return `${chrCode}:${startBp}-${endBp}`;
}
