import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import "lightgallery/css/lightgallery-bundle.css";
import lgThumbnail from "lightgallery/plugins/thumbnail";
import lgZoom from "lightgallery/plugins/zoom";
import LightGallery from "lightgallery/react";
import "overlayscrollbars/overlayscrollbars.css";
import {useEffect, useState} from "react";
import "./app.css";
import {OverlayScrollbarsComponent} from "overlayscrollbars-react";

function App() {
    const [wallpapers, setWallpapers] = useState<string[]>([]);

    useEffect(() => {
        invoke<string[]>("get_wallpapers").then((wallpapers) =>
            setWallpapers(wallpapers),
        );

        invoke<string[]>("update_wallpapers").then((wallpapers) =>
            setWallpapers(wallpapers),
        );
    }, []);

    const getName = (path: string): string => {
        const match = path.trim().match(/[^/\\]+$/);
        return match ? match[0] : "";
    };

    return (
        <>
            <OverlayScrollbarsComponent defer>
                <div className={"w-screen h-screen"}>
                    <LightGallery
                        plugins={[lgZoom, lgThumbnail]}
                        speed={500}
                        thumbnail={true}
                    >
                        {wallpapers.map((v) => (
                            <a href={convertFileSrc(v)} className={"p-3"}>
                                <img
                                    src={convertFileSrc(v)}
                                    alt={getName(v)}
                                    width={240}
                                    height={135}
                                    className={"rounded hover:brightness-125 hover:drop-shadow"}
                                />
                            </a>
                        ))}
                    </LightGallery>
                </div>
            </OverlayScrollbarsComponent>
        </>
    );
}

export default App;
