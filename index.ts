import { blurhash_to_css } from './pkg/blurhash_to_css';

export interface BlurhashCss {
  backgroundImage: string;
  backgroundPosition: string;
  backgroundSize: string;
  backgroundRepeat: string;
  filter: string;
  transform: string;
}

export function blurhashToCss(
  blurhash: string,
  options?: { width: number; height: number }
): BlurhashCss {
  const height = options?.height || 10;
  const width = options?.width || 10;
  return JSON.parse(blurhash_to_css(blurhash, width, height)) as BlurhashCss;
}
