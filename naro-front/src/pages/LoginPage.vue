<script setup lang="ts">
import { computed, ref } from "vue"

const display_id = ref<string>("")
const password = ref<string>("")
const disable = ref<boolean>(false)
const blank = computed(() => display_id.value.length === 0 || password.value.length === 0)

const login = async () => {
		disable.value = true
		let res = await fetch("/api/login", {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				display_id: display_id.value,
				password: password.value
			})
		})
		if(res.ok){
			window.location.href = "/me"
		}
		disable.value = false
	}
</script>

<template>
	<div class="login">
		<h1>This is an login page</h1>
		<div>
			<label for="display_id">Display ID</label>
			<input type="text" v-model="display_id" />
			<label for="password">Password</label>
			<input type="password" v-model="password" />
		</div>
		<div>
			<button :disabled="disable || blank" @click="login">login</button>
		</div>
	</div>
</template>
