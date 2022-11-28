import combineIllionParts from "./combineIllionParts";
import getIllionPartNumbers from "./getIllionPartNumbers";
import getIllionParts from "./getIllionParts";
import hundreds from "./hundreds";
import { WordOptions } from "./toWords";

export function illionWord(illionNumber: number | bigint): string {
	const illionPartNumbers = getIllionPartNumbers(illionNumber);

	return combineIllionParts(
		getIllionParts(illionPartNumbers),
		illionPartNumbers
	)
}

export default function illions(
	digits: string,
	illion: number,
	options: WordOptions
): string {
	return `${hundreds(digits.padStart(3, "0"), options)} ${illionWord(illion)}`;
}
