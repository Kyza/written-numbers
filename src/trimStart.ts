export default function trimStart(str: string, character: string): string {
	let i = 0;
	for (; i < str.length; i++) {
		if (str[i] !== character) break;
	}
	return str.slice(i);
}
