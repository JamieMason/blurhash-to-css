const rust = require('../native');

export function blurhashToCss({
  blurhash,
  height,
  width,
}: {
  blurhash: string;
  height: number;
  width: number;
}): {
  backgroundImage: string;
  backgroundPosition: string;
  backgroundSize: string;
  backgroundRepeat: string;
  filter: string;
  transform: string;
} {
  return rust.blurhashToCss(blurhash, width, height);
}
