<script lang="ts">
    import { Pages, Socials } from '../config';
    import Icon from './Icon.svelte';
    import HLine from './HLine.svelte';

    import { page as currentPage } from '$app/stores';

    let showBurgerMenu = false;
    const handleBurgerClick = () => (showBurgerMenu = !showBurgerMenu);
</script>

<div class="w-full">
    <div class="max-w-6xl mx-auto px-4">
        <div class="flex h-16 md:h-32">
            <div class="flex-none m-4 flex items-center">
                <a href="/" class="">
                    <h1
                        class="text-2xl pr-4 border-solid border-r-2 border-white"
                    >
                        Sebastian Nielsen
                    </h1>
                </a>
            </div>
            <!-- Burger -->
            <div class="flex flex-1 md:hidden items-center">
                {#if Pages.length > 1}
                    <button
                        class="text-gray-400 hover:text-white focus:outline-none focus:text-white"
                        on:click={handleBurgerClick}
                    >
                        <span class="sr-only">Open main menu</span>
                        <svg
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h16m-7 6h7"
                            ></path>
                        </svg>
                    </button>
                {/if}
            </div>

            <div class="flex flex-1 items-center max-md:hidden">
                {#each Pages as page}
                    <a
                        class="px-2 {$currentPage.url.pathname == page.href
                            ? 'underline'
                            : ''}"
                        href={page.href}
                    >
                        {page.title}
                    </a>
                {/each}
            </div>

            <!-- Right -->
            <div class="flex items-center">
                {#each Socials as social}
                    <a
                        href={social.link}
                        class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md text-sm font-medium"
                    >
                        <Icon
                            classes="w-8 mr-4"
                            svg={social.icon}
                            alt={social.alt}
                        />
                    </a>
                {/each}
            </div>
        </div>
    </div>

    <!-- Burger menu -->
    {#if Pages.length > 1}
        <div class={`lg:hidden ${showBurgerMenu ? '' : 'hidden'}`}>
            <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3">
                {#each Pages as page}
                    <a
                        on:click={handleBurgerClick}
                        class="block px-3 py-1 rounded-md text-base font-medium {$currentPage
                            .url.pathname == page.href
                            ? 'underline'
                            : ''}"
                        href={page.href}
                    >
                        {page.title}</a
                    >
                {/each}
            </div>
        </div>
    {/if}
    <HLine />
</div>
