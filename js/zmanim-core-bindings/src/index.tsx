import ZmanimCoreBindings from './NativeZmanimCoreBindings';

export function multiply(a: number, b: number): number {
  return ZmanimCoreBindings.multiply(a, b);
}
