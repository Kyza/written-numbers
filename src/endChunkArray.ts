export default /*
	Chunks an array starting from the end.
*/ function endChunkArray<T>(array: T[], chunkSize: number): T[][] {
	const chunks: T[][] = [];
	for (let i = array.length; i > 0; i -= chunkSize) {
		chunks.push(array.slice(Math.max(i - chunkSize, 0), i));
	}
	return chunks;
}
