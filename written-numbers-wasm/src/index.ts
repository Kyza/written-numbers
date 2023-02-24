// @ts-ignore
import init, { to_words } from "./wasm";

export default init;
// @ts-ignore
export * from "./wasm";

export function toWords({
	number,
	options,
	languageOptions,
}: {
	number: string | bigint | number;
	options?: {
		language: "en" | string;
	};
	languageOptions?: Record<string, any>;
}) {
	number ??= "";
	options ??= { language: "en" };
	languageOptions ??= {};

	const processedLanguageOptions: Record<string, string> = {};
	for (const [key, val] of Object.entries(languageOptions)) {
		processedLanguageOptions[snakeCase(key)] = val.toString();
	}

	// @ts-ignore
	console.log(processedLanguageOptions);

	return to_words(number, options, processedLanguageOptions);
}

function snakeCase(str: string) {
	return str.replace(/[A-Z]/g, (s) => `_${s.toLowerCase()}`);
}
