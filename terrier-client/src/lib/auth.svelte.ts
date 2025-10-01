import { getContext, setContext } from "svelte";
import type { components } from "./schema";

type UserInfo = components["schemas"]["UserInfo"];

const AUTH_KEY = Symbol("auth");
const HACKATHON_KEY = Symbol("hackathon");

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

export function setHackathonContext() {
	let ctx = $state({
		hackathonId: null as string | null,
		hackathonRole: null as string | null,
	});

	setContext(HACKATHON_KEY, ctx);
	return ctx;
}

export function getHackathonContext() {
	return getContext<ReturnType<typeof setHackathonContext>>(HACKATHON_KEY);
}
