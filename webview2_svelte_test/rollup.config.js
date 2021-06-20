import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import svelte from "rollup-plugin-svelte";

const release = !process.env.ROLLUP_WATCH;

/** @type {import("rollup").RollupOptions} */
const config = {
	input: "client/main.js",
	output: {
		format: "iife",
		name: "app",
		file: `target/${release ? "release" : "debug"}/client/bundle.js`,
	},

	plugins: [
		resolve(),
		commonjs(),

		svelte({
			compilerOptions: {
				dev: !release,
			}
		}),
	],
};

export default config;
