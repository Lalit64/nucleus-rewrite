<script type="ts">
    import {platform} from '@tauri-apps/api/os';
    import {appWindow} from '@tauri-apps/api/window'
    import Menu from "./Menu/Menu.svelte";

    let platformName;

    const getPlatform = async () => {
        platformName = await platform();
    }
</script>

<div use:getPlatform>
    <div class="header h-10 w-full flex items-center justify-between border-b border-b-neutral-700"
         data-tauri-drag-region>
        <div class="logo-menu flex items-center justify-center">
            <div class="h-10 p-2">
                <div class="logo h-full flex items-center ml-20">
                    <div class="h-full justify-between aspect-square pr-2">
                        <img alt="" src="/assets/images/Icon_Titlebar.png">
                    </div>
                </div>
            </div>
            <Menu />
        </div>
        {#if platformName === "darwin"}
            <div class="workspace h-full  items-center justify-between -ml-40 hidden lg:flex">
                <button class="px-2 flex items-center justify-between">
                    <a class="px-2">
                        nucleus-rewrite
                    </a>
                    <img
                            src="https://api.iconify.design/fluent:chevron-down-16-filled.svg"
                            alt="minimize"
                            class="dark:invert"
                    />
                </button>
            </div>
            <div class="h-full window-controls min-w[200px] flex ">
                <button class="h-full px-8 dark:hover:bg-neutral-800 hover:bg-gray-200 transition duration-500 flex items-center justify-center"
                        on:click={() => appWindow.minimize()}>
                    <img
                            src="https://api.iconify.design/fluent:subtract-16-filled.svg"
                            alt="minimize"
                            class="dark:invert min-w-fit"
                    />
                </button>
                <button class="h-full px-8 dark:hover:bg-neutral-800 hover:bg-gray-200 transition duration-500 flex items-center justify-cente"
                        on:click={() => appWindow.toggleMaximize()}>
                    <img
                            src="https://api.iconify.design/fluent:maximize-16-filled.svg"
                            alt="maximize"
                            class="dark:invert min-w-fit"
                    />
                </button>
                <button class="h-full px-8 hover:bg-red-600 group transition duration-500 flex items-center justify-center"
                        on:click={() => appWindow.close()}>
                    <img
                            src="https://api.iconify.design/mdi:close.svg"
                            alt="close" class="group-hover:invert dark:invert transition duration-500 min-w-fit"/>
                </button>
            </div>
        {:else }
            <div class="workspace h-full min-w-[90px] flex items-center justify-between">
                <div class="border-l border-l-neutral-700 h-3/5">

                </div>
                [workspace]
            </div>
            <div class="placeholder-box min-w-[200px] ">
            </div>
        {/if}
    </div>
</div>
