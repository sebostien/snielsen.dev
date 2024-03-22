<script lang="ts">
    import '@/app.css';
    import Footer from '@/lib/Footer.svelte';
    import NavBar from '@/lib/NavBar.svelte';
</script>

<div class="flex flex-col min-h-screen">
    <NavBar />
    <div class="w-full py-4 mb-auto">
        <main class="max-w-6xl mx-auto px-8 py-8">
            <slot />
        </main>
    </div>
    <Footer />
</div>
