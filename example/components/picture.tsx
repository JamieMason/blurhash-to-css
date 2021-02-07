import Head from 'next/head';
import React from 'react';
import css from './picture.module.css';

const sizeMappings: Record<string, number> = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
};

const getBreakpoint = function (breakpoint: string): number {
  return sizeMappings[breakpoint];
};

export interface Props {
  className?: string;
  src: string;
  sm?: string;
  md?: string;
  lg?: string;
  xl?: string;
  width: number;
  height: number;
  alt?: string;
  loading?: 'lazy' | 'eager';
}

export const Picture: React.FC<Props> = ({
  className,
  src,
  sm,
  md,
  lg,
  xl,
  width,
  height,
  alt = '',
  loading = 'lazy',
}) => {
  const images = [
    { bp: 'xl', path: xl },
    { bp: 'lg', path: lg },
    { bp: 'md', path: md },
    { bp: 'sm', path: sm },
  ];
  const filteredImages = images.filter(({ path }) => path);

  return (
    <>
      {loading !== 'lazy' ? (
        <Head>
          {filteredImages.map(({ bp, path }, k) => (
            <link
              rel="preload"
              href={path}
              as="image"
              media={`(min-width: ${getBreakpoint(bp)}px)`}
              key={k}
            />
          ))}
        </Head>
      ) : null}
      <picture className={className}>
        {filteredImages.map(({ bp, path }, k) => (
          <source srcSet={path} media={`(min-width: ${getBreakpoint(bp)}px)`} key={k} />
        ))}
        <img
          className={css.img}
          src={src}
          alt={alt}
          width={width}
          height={height}
          loading={loading}
        />
      </picture>
    </>
  );
};
