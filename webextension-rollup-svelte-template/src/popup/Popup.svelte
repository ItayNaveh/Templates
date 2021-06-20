<script lang="ts">
	let count: number = 0;
</script>

<span>{ count }</span> <br />
<button on:click={() => count++}>+</button>

<style lang="scss">
	span {
		color: blue;
		font-family: Helvetica;
	}

	button {
		border-radius: 20%;
		font-family: Helvetica;
	}
</style>
