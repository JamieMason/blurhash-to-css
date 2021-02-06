import wasm from '../Cargo.toml';

export interface BlurhashCss {
  backgroundImage: string;
  backgroundPosition: string;
  backgroundSize: string;
  backgroundRepeat: string;
  filter: string;
  transform: string;
}

export async function blurhashToCss({
  blurhash,
  width,
  height,
}: {
  blurhash: string;
  width: number;
  height: number;
}): Promise<BlurhashCss> {
  const exports = await wasm();
  const json = exports.blurhash_to_css(blurhash, width, height);
  return JSON.parse(json) as BlurhashCss;
}
