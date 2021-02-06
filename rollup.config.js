import rust from '@wasm-tool/rollup-plugin-rust';
import rollupTypescript from '@rollup/plugin-typescript';

export default {
  input: {
    index: 'src/index.ts',
  },
  output: {
    dir: 'dist',
    format: 'cjs',
  },
  watch: false,
  plugins: [
    rust({
      debug: false,
      inlineWasm: true,
      nodejs: true,
      verbose: true,
    }),
    rollupTypescript(),
  ],
};
