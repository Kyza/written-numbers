export default function abs<T extends number | bigint>(num: T): T {
	return num < 0 ? (-num as T) : num;
}
