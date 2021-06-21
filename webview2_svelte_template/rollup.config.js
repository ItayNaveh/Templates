import resolve from "@rollup/plugin-node-resolve";

import typescript from "@rollup/plugin-typescript";
import css from "rollup-plugin-css-only";

import sveltePreprocess from "svelte-preprocess";
import svelte from "rollup-plugin-svelte";
import { terser } from "rollup-plugin-terser";


const release = !process.env.ROLLUP_WATCH;

/** @type {import("rollup").RollupOptions} */
const config = {
	input: "client/main.ts",
	output: {
		format: "iife",
		name: "app",
		file: `target/${release ? "release" : "debug"}/client/bundle.js`,
	},

	plugins: [
		resolve({ browser: true }),

		typescript(),
		css({ output: "bundle.css" }),

		svelte({
			preprocess: sveltePreprocess(),
			compilerOptions: {
				dev: !release,
			}
		}),

		release && terser()
	],
};

export default config;
