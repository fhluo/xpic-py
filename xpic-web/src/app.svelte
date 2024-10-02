<script lang="ts">
    import {convertFileSrc, invoke} from "@tauri-apps/api/core";
    import {getCurrentWindow, LogicalSize} from "@tauri-apps/api/window";
    import {basename} from "@tauri-apps/api/path";
    import 'overlayscrollbars/overlayscrollbars.css';
    import {OverlayScrollbarsComponent} from "overlayscrollbars-svelte";
    import {open} from "@tauri-apps/plugin-shell";
    import {Copy, ExternalLink, FolderOpen, Image as ImageIcon, Save} from "lucide-svelte";
    import {save} from "@tauri-apps/plugin-dialog";
    import {copyFile, readFile} from "@tauri-apps/plugin-fs";
    import {ContextMenu} from "bits-ui";
    import {menu} from "@tauri-apps/api";
    import {writeImage} from "@tauri-apps/plugin-clipboard-manager";
    import {Image, transformImage} from "@tauri-apps/api/image";

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

    const appWindow = getCurrentWindow()

    // get and update wallpapers
    $effect(() => {
        invoke<string[]>("get_wallpapers").then(r => {
            wallpapers = r
            menus = new Array(r.length).fill(false)
        });

        invoke<string[]>("update_wallpapers").then(r =>
            r.filter(v => !wallpapers.includes(v)).forEach(
                v => {
                    wallpapers.push(v)
                    menus.push(false)
                }
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

    function saveWallpaper(path: string) {
        save({
            filters: [{
                name: "Image",
                extensions: ["png", "jpg", "jpeg"]
            }]
        }).then(filename => {
            if (filename) {
                copyFile(path, filename)
            }
        })
    }

    function closeOtherMenus(i: number) {
        for (let j = 0; j < menus.length; j++) {
            if (i !== j) {
                menus[j] = false
            }
        }
    }

    function showInExplorer(path: string) {
        invoke("show_path_in_file_manager", {path})
    }

    function setAsDesktopWallpaper(path: string) {
        invoke("set_as_desktop_wallpaper", {path})
    }

    function copyImageToClipboard(path: string) {
        invoke("copy_image", {path})
    }
</script>

<main>
  <OverlayScrollbarsComponent defer>
    <div class="w-screen h-screen grid items-center justify-center">
      <div id="gallery" bind:this={gallery}
           class="grid grid-cols-4 gap-8 items-center justify-center py-8 px-16"
      >
        {#each wallpapers as path, i}
          <ContextMenu.Root bind:open={menus[i]} onOpenChange={value => {if (value) {closeOtherMenus(i)}}}>
            <ContextMenu.Trigger>
              <img src={convertFileSrc(path)}
                   alt={names[path]}
                   width={config.img.width}
                   height={config.img.height}
                   class="wallpaper"
                   ondblclick={() => void open(path)}
              />
            </ContextMenu.Trigger>
            <ContextMenu.Content class="menu">
              <ContextMenu.Item class="menu-item" onclick={() => void open(path)}>
                <div class="flex flex-row justify-center items-center gap-3">
                  <ExternalLink strokeWidth={1.5} size={20}/>
                  <div>Open Wallpaper</div>
                </div>
              </ContextMenu.Item>
              <ContextMenu.Item class="menu-item" onclick={() => {showInExplorer(path)}}>
                <div class="flex flex-row justify-center items-center gap-3">
                  <FolderOpen strokeWidth={1.5} size={20}/>
                  <div>Show In Explorer</div>
                </div>
              </ContextMenu.Item>
              <ContextMenu.Separator class="menu-separator"/>
              <ContextMenu.Item class="menu-item" onclick={() => {copyImageToClipboard(path)}}>
                <div class="flex flex-row justify-center items-center gap-3">
                  <Copy strokeWidth={1.5} size={20}/>
                  <div>Copy Image</div>
                </div>
              </ContextMenu.Item>
              <ContextMenu.Item class="menu-item" onclick={()=>{saveWallpaper(path)}}>
                <div class="flex flex-row justify-center items-center gap-3">
                  <Save strokeWidth={1.5} size={20}/>
                  <div>Save Wallpaper</div>
                </div>
              </ContextMenu.Item>
              <ContextMenu.Separator class="menu-separator"/>
              <ContextMenu.Item class="menu-item" onclick={() => {setAsDesktopWallpaper(path)}}>
                <div class="flex flex-row justify-center items-center gap-3">
                  <ImageIcon strokeWidth={1.5} size={20}/>
                  <div>Set As Desktop Wallpaper</div>
                </div>
              </ContextMenu.Item>
            </ContextMenu.Content>
          </ContextMenu.Root>
        {/each}
      </div>
    </div>
  </OverlayScrollbarsComponent>
</main>
