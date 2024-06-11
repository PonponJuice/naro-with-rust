<script setup lang="ts">
const logout = async () => {
		let res = await fetch("/api/logout", {
			method: "POST",
			headers: {
				"Content-Type": "application/text"
			}
		})
		if(res.ok){
			window.location.href = "/login"
		}
	}

</script>

<template>
	<main>
		<div :class="$style.container">
			<header :class="$style.header">
				<router-link to="/">Home</router-link>
				|
				<router-link to="/ping">Ping</router-link>
				|
				<router-link to="/signup">Signup</router-link>
				|
				<router-link to="/login">Login</router-link>
				|
				<router-link to="/me">Me</router-link>
				|...|
				<button @click="logout">logout</button>
			</header>

			<router-view />
		</div>
	</main>
</template>

<style module>
.container {
	max-width: fit-content;
	margin: auto;
}
.header {
	display: flex;
	justify-content: center;
}
</style>
