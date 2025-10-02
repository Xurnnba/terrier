import { getContext, setContext } from "svelte";
import type { components } from "./schema";

type UserInfo = components["schemas"]["UserInfo"];

export const apiUrl =
	import.meta.env.VITE_API_URL || "http://localhost:8080/api";

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

export function login(currentPath: string) {
	const appUrl = window.location.origin;
	const encodedUri = encodeURIComponent(`${appUrl}${currentPath}`);

	window.location.href = `${apiUrl}/auth/login?redirect_uri=${encodedUri}`;
}

export function logout() {
	window.location.href = `${apiUrl}/auth/logout`;
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
