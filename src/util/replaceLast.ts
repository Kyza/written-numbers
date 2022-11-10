export default function replaceLast(
	str: string,
	target: string,
	replacement: string
) {
	const index = str.lastIndexOf(target);
	if (index >= 0 && index + target.length >= str.length) {
		str = str.substring(0, index) + replacement;
	}
	return str;
}
