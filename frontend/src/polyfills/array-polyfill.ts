/* eslint-disable import/no-anonymous-default-export */
/* eslint-disable no-extend-native */
// https://stackoverflow.com/questions/37640785/how-do-you-add-polyfills-to-globals-in-typescript-modules
export default {};

declare global {
	interface Array<T> {
		naturalSort(): Array<T>;
		limit(length?: number): Array<T>;
		isEmpty(): boolean;
		last(): T;
	}
}

if (!Array.prototype.limit) {
	Array.prototype.limit = function (length?: number) {
		return this.slice(0, length);
	};
}

if (!Array.prototype.isEmpty) {
	Array.prototype.isEmpty = function () {
		return this.length === 0;
	};
}

if (!Array.prototype.last) {
	Array.prototype.last = function () {
		return this[this.length - 1];
	};
}
