import { TH_LIST } from "./constants";
import toWords from "./toWords";

export default function toOrdinal(num: bigint | number | string): string {
	let words = toWords(num);

	// Special cases.
	if (words.endsWith("one")) return replaceLast(words, "one", "first");
	if (words.endsWith("two")) return replaceLast(words, "two", "second");
	if (words.endsWith("three")) return replaceLast(words, "three", "third");
	if (words.endsWith("five")) return replaceLast(words, "five", "fifth");
	if (words.endsWith("eight")) return replaceLast(words, "eight", "eighth");
	if (words.endsWith("nine")) return replaceLast(words, "nine", "ninth");
	if (words.endsWith("twelve"))
		return replaceLast(words, "twelve", "twelfth");

	// Just in case check to make sure it's a valid word to ordinalify.
	if (TH_LIST.some((ending) => words.endsWith(ending))) return `${words}th`;

	return words;
}
