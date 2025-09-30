import { getContext, setContext } from "svelte";
import type { components } from "./schema";

type UserInfo = components["schemas"]["UserInfo"];

const AUTH_KEY = Symbol("auth");

export function setAuthContext() {
	let auth = $state({
		user: null as UserInfo | null,
		isLoading: true,
	});

	setContext(AUTH_KEY, auth);
	return auth;
}

export function getAuthContext() {
	return getContext<ReturnType<typeof setAuthContext>>(AUTH_KEY);
}
