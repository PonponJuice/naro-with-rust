<script setup lang="ts">
import { onMounted, ref } from "vue"

const username = ref<string>("no data")
const display_id = ref<string>("no data")

onMounted(async () => {
	const res = await fetch("/api/me")
	if (res.ok) {
		let body = JSON.parse(await res.text())
		username.value = body.username
		display_id.value = body.display_id
	}
})
</script>

<template>
	username : {{ username }} <br />
	display_id : {{ display_id }}
</template>
