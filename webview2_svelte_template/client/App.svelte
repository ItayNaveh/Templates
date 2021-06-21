<script lang="ts">
	let count: number = 0;
</script>

<h1>Hey { count } 🥞</h1>
<button on:click={() => count++}>increase</button>

<style>
	h1 {
		font-family: Helvetica;
	}

	button {
		font-family: "Comic Sans MS";
		font-size: xx-large;
	}
</style>
