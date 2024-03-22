<script lang="ts">
    import Icon from '@/lib/Icon.svelte';
    import NavBar from '@/lib/NavBar.svelte';
</script>

<div class="m-auto h-screen flex flex-col max-w-3xl">
    <NavBar />
    <main class=" flex-grow"><slot /></main>
    <footer class="p-4 text-center">2024</footer>
</div>
