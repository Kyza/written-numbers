export default /*
    Chunks an array starting from the end.
*/ function endChunkArray(array, chunkSize) {
    const chunks = [];
    for (let i = array.length; i > 0; i -= chunkSize) {
        chunks.push(array.slice(Math.max(i - chunkSize, 0), i));
    }
    return chunks;
}
