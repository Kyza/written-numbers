import combineIllionParts from "./combineIllionParts";
import getIllionPartNumbers from "./getIllionPartNumbers";
import getIllionParts from "./getIllionParts";

export default function illionWord(illionNumber: number | bigint): string {
	const illionPartNumbers = getIllionPartNumbers(illionNumber);

	return combineIllionParts(
		getIllionParts(illionPartNumbers),
		illionPartNumbers
	);
}
