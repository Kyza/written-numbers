import combineIllionParts from "./combineIllionParts";
import getIllionPartNumbers from "./getIllionPartNumbers";
import getIllionParts from "./getIllionParts";
import hundreds from "./hundreds";
import { WordOptions } from "./toWords";

export default function illions(
	digits: string,
	illion: number,
	options: WordOptions
): string {
	const illionPartNumbers = getIllionPartNumbers(illion);

	return `${hundreds(digits.padStart(3, "0"), options)} ${combineIllionParts(
		getIllionParts(illionPartNumbers),
		illionPartNumbers
	)}`;
}
