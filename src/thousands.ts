import hundreds from "./hundreds";
import { WordOptions } from "./toWords";

export default function thousands(
	digits: string,
	options: WordOptions
): string {
	let result = `${hundreds(digits.padStart(3, "0"), options)} thousand`;
	return result;
}
