import replaceLast from "./util/replaceLast";

export default function toOrdinal(words: string): string {
	// Special cases.
	if (words.endsWith("one")) return replaceLast(words, "one", "first");
	if (words.endsWith("two")) return replaceLast(words, "two", "second");
	if (words.endsWith("three")) return replaceLast(words, "three", "third");
	if (words.endsWith("five")) return replaceLast(words, "five", "fifth");
	if (words.endsWith("eight")) return replaceLast(words, "eight", "eighth");
	if (words.endsWith("nine")) return replaceLast(words, "nine", "ninth");
	if (words.endsWith("twelve"))
		return replaceLast(words, "twelve", "twelfth");

	// Handle twenty through ninety.
	if (words.endsWith("y")) return replaceLast(words, "y", "ieth");

	return `${words}th`;
}
