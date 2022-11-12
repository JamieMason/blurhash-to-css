import { blurhash_to_css, blurhashes_to_css } from './pkg/blurhash_to_css';

export interface BlurhashCss {
  backgroundImage: string;
  backgroundPosition: string;
  backgroundSize: string;
  backgroundRepeat: string;
  filter: string;
  transform: string;
}

export interface Options {
  width: number;
  height: number;
}

export type BlurhashToCss = {
  (blurhash: string, options?: Options): BlurhashCss;
  (blurhashes: string[], options?: Options): BlurhashCss[];
};

export const blurhashToCss: BlurhashToCss = (blurhash, options) => {
  const height = options?.height || 10;
  const width = options?.width || 10;
  const json = Array.isArray(blurhash)
    ? blurhashes_to_css(blurhash, width, height)
    : blurhash_to_css(blurhash, width, height);
  return JSON.parse(json);
};
