// @ts-ignore
import init, { to_words } from "./wasm";

export default init;
// @ts-ignore
export * from "./wasm";

export type ValidNumber = string | bigint | number;
export type Options = {
	language: "en" | string;
};
export type LanguageOptions = Record<string, any>;

export function toWords({
	number,
	options,
	languageOptions,
}: {
	number: ValidNumber;
	options?: Options;
	languageOptions?: LanguageOptions;
}) {
	number ??= "";
	options ??= { language: "en" };
	languageOptions ??= {};

	const processedLanguageOptions: Record<string, string> = {};
	for (const [key, val] of Object.entries(languageOptions)) {
		processedLanguageOptions[snakeCase(key)] = val.toString();
	}

	return to_words(number.toString(), options, processedLanguageOptions);
}

function snakeCase(str: string) {
	return str.replace(/[A-Z]/g, (s) => `_${s.toLowerCase()}`);
}
