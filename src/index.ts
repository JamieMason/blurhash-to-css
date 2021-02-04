import sharp from "sharp";
import { decode } from "blurhash";
import { getPixelsCSS } from "@plaiceholder/css";

export interface BlurhashCss {
  backgroundImage: string;
  backgroundPosition: string;
  backgroundSize: string;
  backgroundRepeat: string;
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
  const pixels = decode(blurhash, width, height, 1);
  const imageBuffer = Buffer.from(pixels);
  const resizedImageBuffer = await sharp(imageBuffer, {
    raw: { channels: 4, height, width },
  })
    .jpeg({ overshootDeringing: true, quality: 40 })
    .toBuffer();
  return getPixelsCSS(resizedImageBuffer);
}
