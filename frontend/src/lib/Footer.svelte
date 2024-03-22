<script lang="ts">
    import HLine from './HLine.svelte';
    import Icon from './Icon.svelte';
</script>

<footer class="w-full p-4 text-center">
    <HLine />
    <p>Sebastian Nielsen</p>
    <p>Gothenburg, Sweden 2024</p>
    <p class="py-4 flex items-center justify-center">
        <Icon classes="inline-block w-6 mr-2" svg="mdiEmail" alt="Email icon" />
        info@snielsen.dev
    </p>
</footer>
