import { blurhashToCss } from 'blurhash-to-css';
import { CssBlurhash } from '../components/css-blurhash';
import type { Photo } from '../fake-data/unsplash';
import { photos } from '../fake-data/unsplash';

export interface Props {
  photos: Photo[];
}

export const getStaticProps = async () => {
  return {
    props: {
      photos: photos.map((photo) => ({
        ...photo,
        style: blurhashToCss(photo.blur_hash),
      })),
    },
  };
};

const IndexPage = ({ photos }: Props) => (
  <main>
    <h1>
      <a href="https://github.com/JamieMason/blurhash-to-css">blurhash-to-css</a> usage example
    </h1>
    <p>
      Here we are using <code>getStaticProps</code> from <a href="https://nextjs.org/">Next.js</a>{' '}
      to fetch images from <a href="https://unsplash.com">unsplash.com</a>, convert their{' '}
      <a href="https://blurha.sh/">BlurHash</a> placeholders to CSS Gradients and display those
      behind each image.
    </p>
    <p>
      <a href="https://css-tricks.com/throttling-the-network/">Throttle the Network in DevTools</a>{' '}
      to slow down image loading and see the placeholders more easily.
    </p>
    <p>
      Find out more at <a href="https://github.com/JamieMason/blurhash-to-css">GitHub</a> or ask me
      on Twitter at <a href="https://twitter.com/fold_left">@fold_left</a>.
    </p>
    {photos.map((photo, i) => (
      <CssBlurhash key={photo.id} loading={i <= 1 ? 'eager' : 'lazy'} photo={photo} />
    ))}
  </main>
);

export default IndexPage;
