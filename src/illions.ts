import hundreds from "./hundreds";
import illionWord from "./illionWord";
import { WordOptions } from "./toWords";

export default function illions(
	digits: string,
	illion: number,
	options: WordOptions
): string {
	return `${hundreds(digits.padStart(3, "0"), options)} ${illionWord(
		illion
	)}`;
}
