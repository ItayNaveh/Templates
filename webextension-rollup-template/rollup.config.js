import typescript from "@rollup/plugin-typescript";

/** @type {import("rollup").RollupOptions[] } */
const config = [
	{
		input: "src/content.ts",
		output: {
			format: "iife",
			dir: "build",
		},
		plugins: [
			typescript(),
		]
	},
	{
		input: "src/background.ts",
		output: {
			format: "iife",
			dir: "build",
		},
		plugins: [
			typescript(),
		]
	}
];

export default config;
