function replaceLast(str, target, replacement) {
    const index = str.lastIndexOf(target);
    if (index >= 0 && index + target.length >= str.length) {
        str = str.substring(0, index) + replacement;
    }
    return str;
}
