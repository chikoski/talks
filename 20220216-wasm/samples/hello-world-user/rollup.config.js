import {nodeResolve} from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import serve from 'rollup-plugin-serve'

const outputDir = 'public'
const production = !process.env.ROLLUP_WATCH;

export default {
  input: 'src/main.ts',
  output: {dir: outputDir, format: 'esm'},
  plugins:
      [
        typescript(),
        nodeResolve(),
        !production && serve({
          contentBase: [outputDir],
          host: 'localhost',
          port: 8080,
        }),
      ]
};