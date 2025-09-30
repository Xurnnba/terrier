export const ROLES = {
	ADMIN: "admin",
	ORGANIZER: "organizer",
	JUDGE: "judge",
	SPONSOR: "sponsor",
	PARTICIPANT: "participant",
	APPLICANT: "applicant",
} as const;

export type Role = (typeof ROLES)[keyof typeof ROLES];

export const ROUTE_PERMISSIONS: Record<string, Role[]> = {
	"/dashboard": ["admin", "organizer", "judge", "sponsor", "participant"],
	"/configuration": ["admin"],
	"/participants": ["admin", "organizer"],
	"/schedule": ["admin", "organizer", "judge", "sponsor", "participant"],
	"/messages": ["admin", "organizer", "judge", "sponsor", "participant"],
	"/judging": ["admin", "organizer", "judge", "sponsor"],
	"/results": ["admin", "organizer", "judge", "sponsor"],
	"/submission": ["admin", "participant"],
	"/check-in": ["admin", "organizer", "participant"],
	"/profile": ["admin", "organizer", "judge", "sponsor", "participant"],
	"/application": ["admin", "applicant"],
};

export const HOME_ROUTES: Record<Role, string> = {
	admin: "/dashboard",
	organizer: "/dashboard",
	judge: "/dashboard",
	sponsor: "/dashboard",
	participant: "/dashboard",
	applicant: "/application",
};

export function canAccessRoute(userRole: string, route: string): boolean {
	const allowedRoles = ROUTE_PERMISSIONS[route];
	if (!allowedRoles) return true;

	return allowedRoles.includes(userRole as Role);
}
