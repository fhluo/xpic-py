<script lang="ts">
    import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
    import {appWindow, LogicalSize} from "@tauri-apps/api/window";
    import {basename} from "@tauri-apps/api/path";
    import 'overlayscrollbars/overlayscrollbars.css';
    import {OverlayScrollbarsComponent} from "overlayscrollbars-svelte";
    import {open} from "@tauri-apps/api/shell";
    import * as ContextMenu from "@lib/components/ui/context-menu";
    import {Download, Image, OpenInNewWindow} from "svelte-radix";

    let wallpapers = $state([] as string[]);
    // get base names for wallpapers
    let names = $derived(
        wallpapers.reduce<{ [key: string]: string }>((r, path) => {
            basename(path).then(name => {
                r[path] = name
            });
            return r
        }, {})
    )

    // get and update wallpapers
    $effect(() => {
        invoke<string[]>("get_wallpapers").then(r => {
            wallpapers = r
        });

        invoke<string[]>("update_wallpapers").then(r =>
            r.filter(v => !wallpapers.includes(v)).forEach(
                v => wallpapers.push(v)
            )
        );
    })

    // disable default context menu
    $effect(() => {
        if (import.meta.env.MODE !== "development") {
            document.addEventListener("contextmenu", (event) =>
                event.preventDefault(),
            );
        }
    })

    let gallery: HTMLDivElement

    const config = {
        default: {
            cols: 4,
            rows: 4,
        },
        img: {
            width: 240,
            height: 135,
        },
        gallery: {
            gap: 32,
            paddingX: 64,
            paddingY: 32,
        },
    }

    // set windows default size and min size
    $effect(() => {
        const defaultWindowSize = new LogicalSize(
            config.default.cols * config.img.width + (config.default.cols - 1) * config.gallery.gap + config.gallery.paddingX * 2,
            config.default.rows * config.img.height + (config.default.rows - 1) * config.gallery.gap + config.gallery.paddingY * 2,
        );
        const minWindowSize = new LogicalSize(
            config.img.width + config.gallery.paddingX * 2,
            config.img.height + config.gallery.paddingY * 2
        );

        appWindow.setSize(defaultWindowSize).then(() => {
            void appWindow.center();
        });
        void appWindow.setMinSize(minWindowSize);
    })

    function computeCols(): Number {
        // cols * width + (cols - 1) * gap + px * 2
        return Math.floor((window.innerWidth - config.gallery.paddingX * 2 + config.gallery.gap) / (config.img.width + config.gallery.gap));
    }

    function adjustCols() {
        gallery.style.gridTemplateColumns = `repeat(${computeCols()}, minmax(0, 1fr))`;
    }

    // set cols
    $effect(() => {
        adjustCols()
        window.addEventListener("resize", adjustCols)
    })

    let menus = $state([] as boolean[])
</script>

<main>
    <OverlayScrollbarsComponent defer>
        <div class="w-screen h-screen grid items-center justify-center">
            <div id="gallery" bind:this={gallery}
                 class="grid grid-cols-4 gap-8 items-center justify-center py-8 px-16"
            >
                {#each wallpapers as path, i}
                    <ContextMenu.Root bind:open={menus[i]} onOpenChange={value => {
                        if (value) {
                            for(let j = 0; j < menus.length; j++) {
                              if (i !== j) {
                                  menus[j] = false
                              }
                            }
                        }
                    }}>
                        <ContextMenu.Trigger>
                            <img src={convertFileSrc(path)}
                                 alt={names[path]}
                                 width={config.img.width}
                                 height={config.img.height}
                                 class="wallpaper rounded hover:brightness-125 hover:drop-shadow select-none"
                                 ondblclick={() => void open(path)}
                            />
                        </ContextMenu.Trigger>
                        <ContextMenu.Content>
                            <ContextMenu.Item onclick={() => void open(path)}>
                                <div class="flex flex-row justify-center items-center gap-2">
                                    <div class="text-gray-600"><OpenInNewWindow/></div>
                                    <div>Open wallpaper</div>
                                </div>
                            </ContextMenu.Item>
                            <ContextMenu.Item>
                                <div class="flex flex-row justify-center items-center gap-2">
                                    <div class="text-gray-600"><Download/></div>
                                    <div>Save wallpaper</div>
                                </div>
                            </ContextMenu.Item>
                            <ContextMenu.Item onclick={() => invoke("set_as_desktop_wallpaper", {path})}>
                                <div class="flex flex-row justify-center items-center gap-2">
                                    <div class="text-gray-600"><Image/></div>
                                    <div>Set as desktop wallpaper</div>
                                </div>
                            </ContextMenu.Item>
                        </ContextMenu.Content>
                    </ContextMenu.Root>
                {/each}
            </div>
        </div>
    </OverlayScrollbarsComponent>
</main>

<style>
</style>
