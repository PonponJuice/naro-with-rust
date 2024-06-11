import { createRouter, createWebHistory } from "vue-router"
import HomePage from "@/pages/HomePage.vue"
import NotFound from "@/pages/NotFound.vue"
import PingPage from "@/pages/PingPage.vue"
import LoginPage from "@/pages/LoginPage.vue"
import SignupPage from "@/pages/SignupPage.vue"
import MePage from "@/pages/MePage.vue"

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: "/",
			name: "home",
			component: HomePage,
			meta: {isPublic: true}
		},
		{
			path: "/ping",
			name: "ping",
			component: PingPage,
			meta: {isPublic: true}
		},
		{
			path: "/signup",
			name: "signup",
			component: SignupPage,
			meta: {isPublic: true}
		},
		{
			path: "/login",
			name: "login",
			component: LoginPage,
			meta: {isPublic: true}
		},
		{
			path: "/me",
			name: "me",
			component: MePage,
			meta: {isPublic: false}
		},
		{
			path: "/:path(.*)",
			name: "about",
			component: NotFound,
			meta: {isPublic: true}
		}
	]
})

router.beforeEach(async (to) => {
	if(to.path === "/signup" || to.path === "/login"){
		const me = await fetch("/api/me")
		if(me.ok){
			return "/me"
		}
	}
	if(to.meta.isPublic){
		return true;
	}
	const me = await fetch("/api/me")
	if(me.ok){
		return true
	}else{
		return "/login"
	}
})

export default router
