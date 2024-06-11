<script setup lang="ts">
import { computed, ref } from "vue"

const username = ref<string>("")
const display_id = ref<string>("")
const password = ref<string>("")
const disable = ref<boolean>(false)
const invalid = computed(() => {
	let invalid = false

	// 文字列の長さの確認
	if(username.value.length === 0 || display_id.value.length === 0 || password.value.length < 8){
		invalid = true
	}
	// パスワードの強度の確認
	if(!/[a-z]/.test(password.value) || !/[A-Z]/.test(password.value) || !/[0-9]/.test(password.value)){
		invalid = true
	}

	return invalid
})

const login = async () => {
		disable.value = true
		console.log(disable)
		let res = await fetch("/api/signup", {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				username: username.value,
				display_id: display_id.value,
				password: password.value
			})
		
		})
		if(res.ok){
			window.location.href = "/login"
		}
		disable.value = false
	}
</script>

<template>
	<div class="signup">
		<h1>This is an Signup page</h1>
		<h2>{{ disable }}</h2>
		<div>
			<label for="display_id">Username</label>
			<input type="text" v-model="username" />
			<label for="display_id">Display ID</label>
			<input type="text" v-model="display_id" />
			<label for="password">Password</label>
			<input type="password" v-model="password" />
		</div>
		<div>
			<button :disabled="disable || invalid" @click="login">singup</button>
		</div>
	</div>
</template>
