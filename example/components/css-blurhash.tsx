import type { Photo } from '../fake-data/unsplash';
import css from './css-blurhash.module.css';
import { Picture, Props as PictureProps } from './picture';

export interface Props {
  loading: PictureProps['loading'];
  photo: Photo;
}

export const CssBlurhash: React.FC<Props> = ({ loading, photo }) => (
  <div className={css.root}>
    <div className={css.blurhash} style={photo.style}></div>
    <Picture
      alt={photo.alt_description}
      className={css.picture}
      height={photo.height}
      lg={photo.urls.full}
      loading={loading}
      md={photo.urls.regular}
      sm={photo.urls.small}
      src={photo.urls.thumb}
      width={photo.width}
    />
  </div>
);
