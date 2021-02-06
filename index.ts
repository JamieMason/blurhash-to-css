import { blurhash_to_css } from './pkg/blurhash_to_css';

export interface BlurhashCss {
  backgroundImage: string;
  backgroundPosition: string;
  backgroundSize: string;
  backgroundRepeat: string;
  filter: string;
  transform: string;
}

export function blurhashToCss({
  blurhash,
  width,
  height,
}: {
  blurhash: string;
  width: number;
  height: number;
}): BlurhashCss {
  return JSON.parse(blurhash_to_css(blurhash, width, height)) as BlurhashCss;
}
