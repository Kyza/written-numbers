import combineIllionParts from "./combineIllionParts";
import getIllionPartNumbers from "./getIllionPartNumbers";
import getIllionParts from "./getIllionParts";
import hundreds from "./hundreds";

export default function illions(digits: string, illion: number): string {
	const illionPartNumbers = getIllionPartNumbers(illion);

	return `${hundreds(digits.padStart(3, "0"))} ${combineIllionParts(
		getIllionParts(illionPartNumbers),
		illionPartNumbers
	)}`;
}
