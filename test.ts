function randomString(
	length: number,
	characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
): string {
	var result = "";
	var charactersLength = characters.length;
	for (var i = 0; i < length; i++) {
		result += characters.charAt(
			Math.floor(Math.random() * charactersLength)
		);
	}
	return result;
}

import { illionWord, toWords } from "./src";

console.log(toWords("9999999999", { and: true, commas: true }));

console.log(illionWord(-123n));
