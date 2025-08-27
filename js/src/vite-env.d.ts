// src/vite-env.d.ts

// When using ?url, Vite returns a string URL
declare module "*.wasm?url" {
  const url: string;
  export default url;
}
