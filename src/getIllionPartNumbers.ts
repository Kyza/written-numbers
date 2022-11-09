import endChunkArray from "./endChunkArray";

export default /*
	Gets the parts of an illion from the illion's number.
	Million is 1.
	Billion is 2.
	Nonillion is 9.
	And so on...
*/ function getIllionPartNumbers(illion: number): string[] {
	// Split the number into chunks of 3 and reverse them.
	// Then reverse the order of those chunks.
	return endChunkArray(illion.toString().split(""), 3)
		.map((chunk) => chunk.reverse().join("").padEnd(3, "0"))
		.reverse();
}
